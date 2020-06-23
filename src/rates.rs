use crate::income::IncomeBracket;

pub fn get_nz_tax () -> Vec<IncomeBracket> {
  return vec![
      IncomeBracket {
          from: 0.0,
          to: Some(14000.0),
          tax: 10.5
      },
      IncomeBracket {
          from: 14000.0,
          to: Some(48000.0),
          tax: 17.5
      },
      IncomeBracket {
          from: 48000.0,
          to: Some(70000.0),
          tax: 30.0
      },
      IncomeBracket {
          from: 70000.0,
          to: None,
          tax: 33.0
      }
  ];
}

pub fn get_sg_tax () -> Vec<IncomeBracket> {
  return vec![
      IncomeBracket {
          from: 20000.0,
          to: Some(30000.0),
          tax: 2.0
      },
      IncomeBracket {
          from: 30000.0,
          to: Some(40000.0),
          tax: 3.5
      },
      IncomeBracket {
          from: 40000.0,
          to: Some(80000.0),
          tax: 7.0
      },
      IncomeBracket {
          from: 80000.0,
          to: Some(120000.0),
          tax: 11.5
      },
      IncomeBracket {
          from: 120000.0,
          to: Some(160000.0),
          tax: 15.0
      },
      IncomeBracket {
          from: 160000.0,
          to: Some(200000.0),
          tax: 18.0
      },
      IncomeBracket {
          from: 200000.0,
          to: Some(40000.0),
          tax: 19.0
      },
      IncomeBracket {
          from: 240000.0,
          to: Some(280000.0),
          tax: 19.5
      },
      IncomeBracket {
          from: 280000.0,
          to: Some(320000.0),
          tax: 20.0
      },
      IncomeBracket {
          from: 320000.0,
          to: None,
          tax: 20.0
      }
  ];
}