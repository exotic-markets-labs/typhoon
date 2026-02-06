use {codama::Codama, typhoon_idl_generator::plugin::TyphoonPlugin};

#[test]
fn idl_test() {
    let codama = Codama::load("./tests/crate")
        .unwrap()
        .add_plugin(TyphoonPlugin);
    let idl = codama.get_json_idl().unwrap();
    let minify_json = |input: &str| -> String {
        let mut out = String::with_capacity(input.len());
        let mut in_string = false;
        let mut escape = false;

        for c in input.chars() {
            if in_string {
                out.push(c);
                if escape {
                    escape = false;
                } else if c == '\\' {
                    escape = true;
                } else if c == '"' {
                    in_string = false;
                }
            } else if c == '"' {
                in_string = true;
                out.push(c);
            } else if !c.is_whitespace() {
                out.push(c);
            }
        }

        out
    };
    println!("{idl}");
    assert_eq!(
        idl,
        minify_json(
            r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "1.0.0",
  "program": {
    "kind": "programNode",
    "name": "test",
    "publicKey": "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
    "version": "1.0.0",
    "accounts": [
      {
        "kind": "accountNode",
        "name": "counter",
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "name": "count",
              "type": {
                "kind": "numberTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "constantDiscriminatorNode",
            "offset": 0,
            "constant": {
              "kind": "constantValueNode",
              "type": {
                "kind": "bytesTypeNode"
              },
              "value": {
                "kind": "bytesValueNode",
                "data": "p4z99AEAAAA=",
                "encoding": "base64"
              }
            }
          }
        ]
      }
    ],
    "instructions": [
      {
        "kind": "instructionNode",
        "name": "initialize",
        "accounts": [
        ],
        "arguments": [
        ],
        "discriminators": [
          {
            "kind": "constantDiscriminatorNode",
            "offset": 0,
            "constant": {
              "kind": "constantValueNode",
              "type": {
                "kind": "numberTypeNode",
                "format": "u8",
                "endian": "le"
              },
              "value": {
                "kind": "numberValueNode",
                "number": 0
              }
            }
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "increment",
        "accounts": [
        ],
        "arguments": [
        ],
        "discriminators": [
          {
            "kind": "constantDiscriminatorNode",
            "offset": 0,
            "constant": {
              "kind": "constantValueNode",
              "type": {
                "kind": "numberTypeNode",
                "format": "u8",
                "endian": "le"
              },
              "value": {
                "kind": "numberValueNode",
                "number": 1
              }
            }
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "close",
        "accounts": [
        ],
        "arguments": [
        ],
        "discriminators": [
          {
            "kind": "constantDiscriminatorNode",
            "offset": 0,
            "constant": {
              "kind": "constantValueNode",
              "type": {
                "kind": "numberTypeNode",
                "format": "u8",
                "endian": "le"
              },
              "value": {
                "kind": "numberValueNode",
                "number": 2
              }
            }
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "init",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "payer",
            "isWritable": true,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "name": "counter",
            "isWritable": true,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "name": "system",
            "isWritable": false,
            "isSigner": false
          }
        ],
        "arguments": [
        ]
      },
      {
        "kind": "instructionNode",
        "name": "counterMut",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "counter",
            "isWritable": true,
            "isSigner": false
          }
        ],
        "arguments": [
        ]
      },
      {
        "kind": "instructionNode",
        "name": "destination",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "destination",
            "isWritable": true,
            "isSigner": false
          }
        ],
        "arguments": [
        ]
      }
    ],
    "definedTypes": [
    ],
    "pdas": [
    ],
    "errors": [
      {
        "kind": "errorNode",
        "name": "error1",
        "code": 0,
        "message": "my custom error"
      }
    ]
  },
  "additionalPrograms": [
  ]
}"#
        )
    )
}
