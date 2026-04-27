use serde_json::Value;

pub fn get_tools_list() -> Value {
    serde_json::json!({
        "tools": [
            {
                "name": "get_system_time",
                "description": "Get current system time in specified timezone",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "timezone": {
                            "type": "string",
                            "description": "IANA timezone name (e.g., 'America/New_York', 'Europe/London'). Defaults to UTC"
                        }
                    },
                    "required": []
                },
                "annotations": {
                    "title": "Get System Time",
                    "readOnlyHint": true,
                    "destructiveHint": false,
                    "idempotentHint": false,
                    "openWorldHint": false
                }
            },
            {
                "name": "convert_time",
                "description": "Convert time between different timezones",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "time": {
                            "type": "string",
                            "description": "Time to convert in RFC3339 format or common formats like '2006-01-02 15:04:05'"
                        },
                        "source_timezone": {
                            "type": "string",
                            "description": "Source IANA timezone name"
                        },
                        "target_timezone": {
                            "type": "string",
                            "description": "Target IANA timezone name"
                        }
                    },
                    "required": ["time", "source_timezone", "target_timezone"]
                },
                "annotations": {
                    "title": "Convert Time",
                    "readOnlyHint": true,
                    "destructiveHint": false,
                    "idempotentHint": true,
                    "openWorldHint": false
                }
            }
        ]
    })
}
