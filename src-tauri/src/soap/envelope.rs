use quick_xml::events::Event;
use quick_xml::reader::Reader;

/// Escapes the characters that are unsafe inside XML text content.
fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Builds the SOAP 1.1 envelope for AzerothCore's `executeCommand` operation.
pub fn build_envelope(command: &str) -> String {
    let escaped = escape_xml(command);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/1999/XMLSchema-instance" xmlns:xsd="http://www.w3.org/1999/XMLSchema">
<SOAP-ENV:Body>
<ns1:executeCommand xmlns:ns1="urn:AC">
<command>{escaped}</command>
</ns1:executeCommand>
</SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#
    )
}

#[derive(Debug)]
pub enum ParsedResponse {
    Result(String),
    Fault(String),
}

/// Parses the SOAP response body, extracting either the `<result>` text on
/// success or the fault message (`faultstring`/`detail`) on failure.
pub fn parse_response(body: &str) -> Result<ParsedResponse, String> {
    let mut reader = Reader::from_str(body);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut current_tag: Option<String> = None;
    let mut result_text: Option<String> = None;
    let mut fault_string: Option<String> = None;
    let mut fault_detail: Option<String> = None;
    let mut is_fault = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(e.name().as_ref());
                if name == "Fault" {
                    is_fault = true;
                }
                current_tag = Some(name);
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().into_owned();
                match current_tag.as_deref() {
                    Some("result") => result_text = Some(text),
                    Some("faultstring") => fault_string = Some(text),
                    Some("detail") | Some("faultdetail") => fault_detail = Some(text),
                    _ => {}
                }
            }
            Ok(Event::End(_)) => {
                current_tag = None;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {e}")),
            _ => {}
        }
        buf.clear();
    }

    if is_fault {
        let message = fault_detail
            .or(fault_string)
            .unwrap_or_else(|| "Unknown SOAP fault".to_string());
        return Ok(ParsedResponse::Fault(message));
    }

    match result_text {
        Some(text) => Ok(ParsedResponse::Result(text)),
        None => Err("Response contained neither a result nor a fault".to_string()),
    }
}

fn local_name(qname: &[u8]) -> String {
    let s = String::from_utf8_lossy(qname);
    match s.rsplit_once(':') {
        Some((_, local)) => local.to_string(),
        None => s.into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_envelope_with_escaped_command() {
        let env = build_envelope(r#"additem "Thunderfury" & <test>"#);
        assert!(env.contains("<command>additem &quot;Thunderfury&quot; &amp; &lt;test&gt;</command>"));
        assert!(env.contains("urn:AC"));
    }

    #[test]
    fn parses_successful_result() {
        let body = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/">
<SOAP-ENV:Body>
<ns1:executeCommandResponse xmlns:ns1="urn:AC">
<result>Connected players: 1.</result>
</ns1:executeCommandResponse>
</SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;
        match parse_response(body).unwrap() {
            ParsedResponse::Result(text) => assert_eq!(text, "Connected players: 1."),
            ParsedResponse::Fault(_) => panic!("expected result, got fault"),
        }
    }

    #[test]
    fn parses_soap_fault() {
        let body = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/">
<SOAP-ENV:Body>
<SOAP-ENV:Fault>
<faultcode>SOAP-ENV:Client</faultcode>
<faultstring>Command can not be empty</faultstring>
<detail>The supplied command was an empty string</detail>
</SOAP-ENV:Fault>
</SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;
        match parse_response(body).unwrap() {
            ParsedResponse::Fault(text) => {
                assert_eq!(text, "The supplied command was an empty string")
            }
            ParsedResponse::Result(_) => panic!("expected fault, got result"),
        }
    }
}
