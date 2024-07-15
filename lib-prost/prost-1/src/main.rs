use prost::Message;




#[derive(Clone, PartialEq, Message)]
pub struct TagsInferred {
    #[prost(bool)]
    pub one: bool,
    #[prost(int32, optional)]
    pub two: Option<i32>,
    #[prost(float, repeated)]
    pub three: Vec<f32>,
}

fn main() {
    let x = TagsInferred {
        one: true,
        two: None,
        three: vec![0.5],
    };
    let mut buf = Vec::new();
    x.encode(&mut buf).unwrap();
    println!("{:?}", buf);
}