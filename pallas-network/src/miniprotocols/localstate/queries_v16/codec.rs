use super::*;
use pallas_codec::minicbor::{data::Tag, decode, encode, Decode, Decoder, Encode, Encoder};

impl<Ctx> Encode<Ctx> for LedgerQuery {
    fn encode<W: encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _: &mut Ctx,
    ) -> Result<(), encode::Error<W::Error>> {
        match self {
            LedgerQuery::BlockQuery(era, q) => {
                e.encode((0, (era, q)))?;
            }
            LedgerQuery::HardForkQuery(q) => {
                e.encode((2, q))?;
            }
        }

        Ok(())
    }
}

impl<'b, Ctx> Decode<'b, Ctx> for LedgerQuery {
    fn decode(d: &mut Decoder<'b>, _: &mut Ctx) -> Result<Self, decode::Error> {
        d.array()?;
        let tag = d.u16()?;
        match tag {
            0 => {
                let (era, q) = d.decode()?;
                Ok(Self::BlockQuery(era, q))
            }
            2 => {
                let q = d.decode()?;
                Ok(Self::HardForkQuery(q))
            }
            _ => Err(decode::Error::message("invalid tag")),
        }
    }
}

impl<'b, C> minicbor::decode::Decode<'b, C> for Value {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        match d.datatype()? {
            minicbor::data::Type::U8 => Ok(Value::Coin(d.decode_with(ctx)?)),
            minicbor::data::Type::U16 => Ok(Value::Coin(d.decode_with(ctx)?)),
            minicbor::data::Type::U32 => Ok(Value::Coin(d.decode_with(ctx)?)),
            minicbor::data::Type::U64 => Ok(Value::Coin(d.decode_with(ctx)?)),
            minicbor::data::Type::Array => {
                d.array()?;
                let coin = d.decode_with(ctx)?;
                let multiasset = d.decode_with(ctx)?;
                Ok(Value::Multiasset(coin, multiasset))
            }
            _ => Err(minicbor::decode::Error::message(
                "unknown cbor data type for Value enum",
            )),
        }
    }
}

impl<C> minicbor::encode::Encode<C> for Value {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            Value::Coin(coin) => {
                e.encode_with(coin, ctx)?;
            }
            Value::Multiasset(coin, other) => {
                e.array(2)?;
                e.encode_with(coin, ctx)?;
                e.encode_with(other, ctx)?;
            }
        };

        Ok(())
    }
}

impl<'b, C> minicbor::decode::Decode<'b, C> for RationalNumber {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.tag()?;
        d.array()?;

        Ok(RationalNumber {
            numerator: d.decode_with(ctx)?,
            denominator: d.decode_with(ctx)?,
        })
    }
}

impl<C> minicbor::encode::Encode<C> for RationalNumber {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.tag(Tag::new(30))?;
        e.array(2)?;
        e.encode_with(self.numerator, ctx)?;
        e.encode_with(self.denominator, ctx)?;

        Ok(())
    }
}

impl<'b, C> minicbor::decode::Decode<'b, C> for TransactionOutput {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        match d.datatype()? {
            minicbor::data::Type::Map => Ok(TransactionOutput::Current(d.decode_with(ctx)?)),
            minicbor::data::Type::Array => Ok(TransactionOutput::Legacy(d.decode_with(ctx)?)),
            _ => Err(minicbor::decode::Error::message(
                "unknown cbor data type for TransactionOutput enum",
            )),
        }
    }
}

impl<C> minicbor::encode::Encode<C> for TransactionOutput {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            TransactionOutput::Current(map) => {
                e.encode_with(map, ctx)?;
            }
            TransactionOutput::Legacy(array) => {
                e.encode_with(array, ctx)?;
            }
        };

        Ok(())
    }
}

impl<'b, C> minicbor::decode::Decode<'b, C> for FilteredDelegsRewards {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.array()?;
        d.array()?;
        Ok(FilteredDelegsRewards {
            delegs: d.decode_with(ctx)?,
            rewards: d.decode_with(ctx)?,
        })
    }
}

impl<C> minicbor::encode::Encode<C> for FilteredDelegsRewards {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.array(1)?;
        e.array(2)?;
        e.encode_with(self.delegs.clone(), ctx)?;
        e.encode_with(self.rewards.clone(), ctx)?;

        Ok(())
    }
}
