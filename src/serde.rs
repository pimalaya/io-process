//! Module dedicated to [`serde`] de/serialization of [`Command`].

use std::fmt;

use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::command::Command;

impl Serialize for Command {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let args = self.get_args();
        let args_len = match &args {
            Some(args) => args.len() + 1,
            None => 0,
        };

        let mut seq = serializer.serialize_seq(Some(args_len))?;

        seq.serialize_element(&self.get_program())?;

        if let Some(args) = args {
            for arg in args {
                seq.serialize_element(&arg)?;
            }
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Command, D::Error> {
        deserializer.deserialize_any(CommandVisitor)
    }
}

struct CommandVisitor;

impl<'de> Visitor<'de> for CommandVisitor {
    type Value = Command;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string (full command) or a list of string (command arguments)")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let mut args = v.split_whitespace();
        let program = args.next().ok_or(E::custom("command cannot be empty"))?;
        let mut command = Command::new(program);
        command.args(args);
        Ok(command)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let program = seq
            .next_element::<String>()?
            .ok_or(<A::Error as Error>::custom("command cannot be empty"))?;

        let mut command = Command::new(program);

        while let Some(arg) = seq.next_element::<String>()? {
            command.arg(arg);
        }

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use serde::{
        de::value::{Error, SeqDeserializer, StringDeserializer},
        Deserialize,
    };

    use crate::command::Command;

    // TODO
    //
    // #[test]
    // fn serialize() {
    //
    // }

    #[test]
    fn deserialize_string() {
        let mut expected = Command::new("program");
        expected.arg("arg1").arg("arg2");

        let s = String::from("program arg1 arg2");
        let s = StringDeserializer::<Error>::new(s);
        let got = Command::deserialize(s).unwrap();
        assert_eq!(expected, got);

        let s = String::from("program   \narg1 \t arg2");
        let s = StringDeserializer::<Error>::new(s);
        let got = Command::deserialize(s).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn deserialize_empty_string() {
        let s = StringDeserializer::<Error>::new(String::new());
        let err = Command::deserialize(s).unwrap_err();
        assert_eq!("command cannot be empty", err.to_string());

        let s = StringDeserializer::<Error>::new(String::from(" \n\t"));
        let err = Command::deserialize(s).unwrap_err();
        assert_eq!("command cannot be empty", err.to_string());
    }

    #[test]
    fn deserialize_seq() {
        let mut expected = Command::new("program");
        expected.arg("arg1").arg("arg2");

        let s = ["program", "arg1", "arg2"];
        let s = SeqDeserializer::<_, Error>::new(s.into_iter());
        let got = Command::deserialize(s).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn deserialize_empty_seq() {
        let s: [&str; 0] = [];
        let s = SeqDeserializer::new(s.into_iter());
        let err: Error = Command::deserialize(s).unwrap_err();
        assert_eq!("command cannot be empty", err.to_string());
    }
}
