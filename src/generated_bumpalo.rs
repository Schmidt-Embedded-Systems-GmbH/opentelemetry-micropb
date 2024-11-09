pub mod opentelemetry_ {
    pub mod proto_ {
        pub mod common_ {
            pub mod v1_ {
                pub mod AnyValue_ {
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Value {
                        StringValue(crate::bumpalo::UnsafeString),
                        BoolValue(bool),
                        IntValue(i64),
                        DoubleValue(f64),
                        ArrayValue(super::ArrayValue),
                        KvlistValue(super::KeyValueList),
                        BytesValue(crate::bumpalo::UnsafeVec<u8>),
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct AnyValue {
                    pub r#value: ::core::option::Option<AnyValue_::Value>,
                }
                impl ::core::default::Default for AnyValue {
                    fn default() -> Self {
                        Self {
                            r#value: ::core::default::Default::default(),
                        }
                    }
                }
                impl AnyValue {}
                impl ::micropb::MessageDecode for AnyValue {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::StringValue(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::StringValue(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                }
                                2u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::BoolValue(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::BoolValue(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    let val = decoder.decode_bool()?;
                                    *mut_ref = val as _;
                                }
                                3u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::IntValue(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::IntValue(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    let val = decoder.decode_int64()?;
                                    *mut_ref = val as _;
                                }
                                4u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::DoubleValue(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::DoubleValue(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    let val = decoder.decode_double()?;
                                    *mut_ref = val as _;
                                }
                                5u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::ArrayValue(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::ArrayValue(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    mut_ref.decode_len_delimited(decoder)?;
                                }
                                6u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::KvlistValue(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::KvlistValue(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    mut_ref.decode_len_delimited(decoder)?;
                                }
                                7u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::BytesValue(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::BytesValue(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    decoder
                                        .decode_bytes(mut_ref, ::micropb::Presence::Explicit)?;
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for AnyValue {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        if let Some(oneof) = &self.r#value {
                            match &*oneof {
                                AnyValue_::Value::StringValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(10u32)?;
                                    encoder.encode_string(val_ref)?;
                                }
                                AnyValue_::Value::BoolValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(16u32)?;
                                    encoder.encode_bool(*val_ref)?;
                                }
                                AnyValue_::Value::IntValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(24u32)?;
                                    encoder.encode_int64(*val_ref as _)?;
                                }
                                AnyValue_::Value::DoubleValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(33u32)?;
                                    encoder.encode_double(*val_ref)?;
                                }
                                AnyValue_::Value::ArrayValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(42u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                                AnyValue_::Value::KvlistValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(50u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                                AnyValue_::Value::BytesValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(58u32)?;
                                    encoder.encode_bytes(val_ref)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        if let Some(oneof) = &self.r#value {
                            match &*oneof {
                                AnyValue_::Value::StringValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                                AnyValue_::Value::BoolValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size += 1usize + 1;
                                }
                                AnyValue_::Value::IntValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize + ::micropb::size::sizeof_int64(*val_ref as _);
                                }
                                AnyValue_::Value::DoubleValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size += 1usize + 8;
                                }
                                AnyValue_::Value::ArrayValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                                AnyValue_::Value::KvlistValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                                AnyValue_::Value::BytesValue(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ArrayValue {
                    pub r#values: crate::bumpalo::UnsafeVec<AnyValue>,
                }
                impl ::core::default::Default for ArrayValue {
                    fn default() -> Self {
                        Self {
                            r#values: ::core::default::Default::default(),
                        }
                    }
                }
                impl ArrayValue {}
                impl ::micropb::MessageDecode for ArrayValue {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: AnyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#values.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ArrayValue {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#values.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#values.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct KeyValueList {
                    pub r#values: crate::bumpalo::UnsafeVec<KeyValue>,
                }
                impl ::core::default::Default for KeyValueList {
                    fn default() -> Self {
                        Self {
                            r#values: ::core::default::Default::default(),
                        }
                    }
                }
                impl KeyValueList {}
                impl ::micropb::MessageDecode for KeyValueList {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#values.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for KeyValueList {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#values.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#values.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                pub mod KeyValue_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `value`
                        #[inline]
                        pub fn r#value(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `value`
                        #[inline]
                        pub fn set_value(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `value`
                        #[inline]
                        pub fn clear_value(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `value`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_value(mut self) -> Self {
                            self.set_value();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct KeyValue {
                    pub r#key: crate::bumpalo::UnsafeString,
                    pub r#value: AnyValue,
                    pub _has: KeyValue_::_Hazzer,
                }
                impl ::core::default::Default for KeyValue {
                    fn default() -> Self {
                        Self {
                            r#key: ::core::default::Default::default(),
                            r#value: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl KeyValue {
                    ///Return a reference to `value` as an `Option`
                    #[inline]
                    pub fn r#value(&self) -> ::core::option::Option<&AnyValue> {
                        self._has.r#value().then_some(&self.r#value)
                    }
                    ///Return a mutable reference to `value` as an `Option`
                    #[inline]
                    pub fn mut_value(
                        &mut self,
                    ) -> ::core::option::Option<&mut AnyValue> {
                        self._has.r#value().then_some(&mut self.r#value)
                    }
                    ///Set the value and presence of `value`
                    #[inline]
                    pub fn set_value(&mut self, value: AnyValue) {
                        self._has.set_value();
                        self.r#value = value.into();
                    }
                    ///Clear the presence of `value`
                    #[inline]
                    pub fn clear_value(&mut self) {
                        self._has.clear_value();
                    }
                }
                impl ::micropb::MessageDecode for KeyValue {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#key;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#value;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_value();
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for KeyValue {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            let val_ref = &self.r#key;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(10u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#value() {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            let val_ref = &self.r#key;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#value() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct InstrumentationScope {
                    pub r#name: crate::bumpalo::UnsafeString,
                    pub r#version: crate::bumpalo::UnsafeString,
                    pub r#attributes: crate::bumpalo::UnsafeVec<KeyValue>,
                    pub r#dropped_attributes_count: u32,
                }
                impl ::core::default::Default for InstrumentationScope {
                    fn default() -> Self {
                        Self {
                            r#name: ::core::default::Default::default(),
                            r#version: ::core::default::Default::default(),
                            r#attributes: ::core::default::Default::default(),
                            r#dropped_attributes_count: ::core::default::Default::default(),
                        }
                    }
                }
                impl InstrumentationScope {}
                impl ::micropb::MessageDecode for InstrumentationScope {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#name;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#version;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                3u32 => {
                                    let mut val: KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                4u32 => {
                                    let mut_ref = &mut self.r#dropped_attributes_count;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for InstrumentationScope {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            let val_ref = &self.r#name;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(10u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#version;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(18u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(26u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(32u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            let val_ref = &self.r#name;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#version;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        size
                    }
                }
            }
        }
        pub mod resource_ {
            pub mod v1_ {
                #[derive(Debug, PartialEq, Clone)]
                pub struct Resource {
                    pub r#attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#dropped_attributes_count: u32,
                }
                impl ::core::default::Default for Resource {
                    fn default() -> Self {
                        Self {
                            r#attributes: ::core::default::Default::default(),
                            r#dropped_attributes_count: ::core::default::Default::default(),
                        }
                    }
                }
                impl Resource {}
                impl ::micropb::MessageDecode for Resource {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#dropped_attributes_count;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Resource {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(16u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        size
                    }
                }
            }
        }
        pub mod logs_ {
            pub mod v1_ {
                #[derive(Debug, PartialEq, Clone)]
                pub struct LogsData {
                    pub r#resource_logs: crate::bumpalo::UnsafeVec<ResourceLogs>,
                }
                impl ::core::default::Default for LogsData {
                    fn default() -> Self {
                        Self {
                            r#resource_logs: ::core::default::Default::default(),
                        }
                    }
                }
                impl LogsData {}
                impl ::micropb::MessageDecode for LogsData {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: ResourceLogs = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#resource_logs.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for LogsData {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#resource_logs.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#resource_logs.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                pub mod ResourceLogs_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `resource`
                        #[inline]
                        pub fn r#resource(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `resource`
                        #[inline]
                        pub fn set_resource(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `resource`
                        #[inline]
                        pub fn clear_resource(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `resource`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_resource(mut self) -> Self {
                            self.set_resource();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ResourceLogs {
                    pub r#resource: super::super::resource_::v1_::Resource,
                    pub r#scope_logs: crate::bumpalo::UnsafeVec<ScopeLogs>,
                    pub r#schema_url: crate::bumpalo::UnsafeString,
                    pub _has: ResourceLogs_::_Hazzer,
                }
                impl ::core::default::Default for ResourceLogs {
                    fn default() -> Self {
                        Self {
                            r#resource: ::core::default::Default::default(),
                            r#scope_logs: ::core::default::Default::default(),
                            r#schema_url: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl ResourceLogs {
                    ///Return a reference to `resource` as an `Option`
                    #[inline]
                    pub fn r#resource(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&self.r#resource)
                    }
                    ///Return a mutable reference to `resource` as an `Option`
                    #[inline]
                    pub fn mut_resource(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&mut self.r#resource)
                    }
                    ///Set the value and presence of `resource`
                    #[inline]
                    pub fn set_resource(
                        &mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) {
                        self._has.set_resource();
                        self.r#resource = value.into();
                    }
                    ///Clear the presence of `resource`
                    #[inline]
                    pub fn clear_resource(&mut self) {
                        self._has.clear_resource();
                    }
                }
                impl ::micropb::MessageDecode for ResourceLogs {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#resource;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_resource();
                                }
                                2u32 => {
                                    let mut val: ScopeLogs = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#scope_logs.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#schema_url;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ResourceLogs {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            if let Some(val_ref) = self.r#resource() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for val_ref in self.r#scope_logs.iter() {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            if let Some(val_ref) = self.r#resource() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for val_ref in self.r#scope_logs.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                pub mod ScopeLogs_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `scope`
                        #[inline]
                        pub fn r#scope(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `scope`
                        #[inline]
                        pub fn set_scope(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `scope`
                        #[inline]
                        pub fn clear_scope(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `scope`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_scope(mut self) -> Self {
                            self.set_scope();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ScopeLogs {
                    pub r#scope: super::super::common_::v1_::InstrumentationScope,
                    pub r#log_records: crate::bumpalo::UnsafeVec<LogRecord>,
                    pub r#schema_url: crate::bumpalo::UnsafeString,
                    pub _has: ScopeLogs_::_Hazzer,
                }
                impl ::core::default::Default for ScopeLogs {
                    fn default() -> Self {
                        Self {
                            r#scope: ::core::default::Default::default(),
                            r#log_records: ::core::default::Default::default(),
                            r#schema_url: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl ScopeLogs {
                    ///Return a reference to `scope` as an `Option`
                    #[inline]
                    pub fn r#scope(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&self.r#scope)
                    }
                    ///Return a mutable reference to `scope` as an `Option`
                    #[inline]
                    pub fn mut_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&mut self.r#scope)
                    }
                    ///Set the value and presence of `scope`
                    #[inline]
                    pub fn set_scope(
                        &mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) {
                        self._has.set_scope();
                        self.r#scope = value.into();
                    }
                    ///Clear the presence of `scope`
                    #[inline]
                    pub fn clear_scope(&mut self) {
                        self._has.clear_scope();
                    }
                }
                impl ::micropb::MessageDecode for ScopeLogs {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#scope;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_scope();
                                }
                                2u32 => {
                                    let mut val: LogRecord = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#log_records.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#schema_url;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ScopeLogs {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            if let Some(val_ref) = self.r#scope() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for val_ref in self.r#log_records.iter() {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            if let Some(val_ref) = self.r#scope() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for val_ref in self.r#log_records.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                pub mod LogRecord_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `body`
                        #[inline]
                        pub fn r#body(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `body`
                        #[inline]
                        pub fn set_body(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `body`
                        #[inline]
                        pub fn clear_body(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `body`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_body(mut self) -> Self {
                            self.set_body();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct LogRecord {
                    pub r#time_unix_nano: u64,
                    pub r#observed_time_unix_nano: u64,
                    pub r#severity_number: SeverityNumber,
                    pub r#severity_text: crate::bumpalo::UnsafeString,
                    pub r#body: super::super::common_::v1_::AnyValue,
                    pub r#attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#dropped_attributes_count: u32,
                    pub r#flags: u32,
                    pub r#trace_id: crate::bumpalo::UnsafeVec<u8>,
                    pub r#span_id: crate::bumpalo::UnsafeVec<u8>,
                    pub _has: LogRecord_::_Hazzer,
                }
                impl ::core::default::Default for LogRecord {
                    fn default() -> Self {
                        Self {
                            r#time_unix_nano: ::core::default::Default::default(),
                            r#observed_time_unix_nano: ::core::default::Default::default(),
                            r#severity_number: ::core::default::Default::default(),
                            r#severity_text: ::core::default::Default::default(),
                            r#body: ::core::default::Default::default(),
                            r#attributes: ::core::default::Default::default(),
                            r#dropped_attributes_count: ::core::default::Default::default(),
                            r#flags: ::core::default::Default::default(),
                            r#trace_id: ::core::default::Default::default(),
                            r#span_id: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl LogRecord {
                    ///Return a reference to `body` as an `Option`
                    #[inline]
                    pub fn r#body(
                        &self,
                    ) -> ::core::option::Option<&super::super::common_::v1_::AnyValue> {
                        self._has.r#body().then_some(&self.r#body)
                    }
                    ///Return a mutable reference to `body` as an `Option`
                    #[inline]
                    pub fn mut_body(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::AnyValue,
                    > {
                        self._has.r#body().then_some(&mut self.r#body)
                    }
                    ///Set the value and presence of `body`
                    #[inline]
                    pub fn set_body(
                        &mut self,
                        value: super::super::common_::v1_::AnyValue,
                    ) {
                        self._has.set_body();
                        self.r#body = value.into();
                    }
                    ///Clear the presence of `body`
                    #[inline]
                    pub fn clear_body(&mut self) {
                        self._has.clear_body();
                    }
                }
                impl ::micropb::MessageDecode for LogRecord {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                11u32 => {
                                    let mut_ref = &mut self.r#observed_time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#severity_number;
                                    {
                                        let val = decoder
                                            .decode_int32()
                                            .map(|n| SeverityNumber(n as _))?;
                                        let val_ref = &val;
                                        if val_ref.0 != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#severity_text;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                5u32 => {
                                    let mut_ref = &mut self.r#body;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_body();
                                }
                                6u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                7u32 => {
                                    let mut_ref = &mut self.r#dropped_attributes_count;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                8u32 => {
                                    let mut_ref = &mut self.r#flags;
                                    {
                                        let val = decoder.decode_fixed32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                9u32 => {
                                    let mut_ref = &mut self.r#trace_id;
                                    {
                                        decoder
                                            .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                10u32 => {
                                    let mut_ref = &mut self.r#span_id;
                                    {
                                        decoder
                                            .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for LogRecord {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(9u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#observed_time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(89u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#severity_number;
                            if val_ref.0 != 0 {
                                encoder.encode_varint32(16u32)?;
                                encoder.encode_int32(val_ref.0 as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#severity_text;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#body() {
                                encoder.encode_varint32(42u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(50u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(56u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                encoder.encode_varint32(69u32)?;
                                encoder.encode_fixed32(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#trace_id;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(74u32)?;
                                encoder.encode_bytes(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#span_id;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(82u32)?;
                                encoder.encode_bytes(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#observed_time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#severity_number;
                            if val_ref.0 != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
                            }
                        }
                        {
                            let val_ref = &self.r#severity_text;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#body() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                size += 1usize + 4;
                            }
                        }
                        {
                            let val_ref = &self.r#trace_id;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#span_id;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct SeverityNumber(pub i32);
                impl SeverityNumber {
                    pub const Unspecified: Self = Self(0);
                    pub const Trace: Self = Self(1);
                    pub const Trace2: Self = Self(2);
                    pub const Trace3: Self = Self(3);
                    pub const Trace4: Self = Self(4);
                    pub const Debug: Self = Self(5);
                    pub const Debug2: Self = Self(6);
                    pub const Debug3: Self = Self(7);
                    pub const Debug4: Self = Self(8);
                    pub const Info: Self = Self(9);
                    pub const Info2: Self = Self(10);
                    pub const Info3: Self = Self(11);
                    pub const Info4: Self = Self(12);
                    pub const Warn: Self = Self(13);
                    pub const Warn2: Self = Self(14);
                    pub const Warn3: Self = Self(15);
                    pub const Warn4: Self = Self(16);
                    pub const Error: Self = Self(17);
                    pub const Error2: Self = Self(18);
                    pub const Error3: Self = Self(19);
                    pub const Error4: Self = Self(20);
                    pub const Fatal: Self = Self(21);
                    pub const Fatal2: Self = Self(22);
                    pub const Fatal3: Self = Self(23);
                    pub const Fatal4: Self = Self(24);
                }
                impl core::default::Default for SeverityNumber {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl core::convert::From<i32> for SeverityNumber {
                    fn from(val: i32) -> Self {
                        Self(val)
                    }
                }
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct LogRecordFlags(pub i32);
                impl LogRecordFlags {
                    pub const DoNotUse: Self = Self(0);
                    pub const TraceFlagsMask: Self = Self(255);
                }
                impl core::default::Default for LogRecordFlags {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl core::convert::From<i32> for LogRecordFlags {
                    fn from(val: i32) -> Self {
                        Self(val)
                    }
                }
            }
        }
        pub mod metrics_ {
            pub mod v1_ {
                #[derive(Debug, PartialEq, Clone)]
                pub struct MetricsData {
                    pub r#resource_metrics: crate::bumpalo::UnsafeVec<ResourceMetrics>,
                }
                impl ::core::default::Default for MetricsData {
                    fn default() -> Self {
                        Self {
                            r#resource_metrics: ::core::default::Default::default(),
                        }
                    }
                }
                impl MetricsData {}
                impl ::micropb::MessageDecode for MetricsData {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: ResourceMetrics = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#resource_metrics.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for MetricsData {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#resource_metrics.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#resource_metrics.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                pub mod ResourceMetrics_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `resource`
                        #[inline]
                        pub fn r#resource(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `resource`
                        #[inline]
                        pub fn set_resource(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `resource`
                        #[inline]
                        pub fn clear_resource(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `resource`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_resource(mut self) -> Self {
                            self.set_resource();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ResourceMetrics {
                    pub r#resource: super::super::resource_::v1_::Resource,
                    pub r#scope_metrics: crate::bumpalo::UnsafeVec<ScopeMetrics>,
                    pub r#schema_url: crate::bumpalo::UnsafeString,
                    pub _has: ResourceMetrics_::_Hazzer,
                }
                impl ::core::default::Default for ResourceMetrics {
                    fn default() -> Self {
                        Self {
                            r#resource: ::core::default::Default::default(),
                            r#scope_metrics: ::core::default::Default::default(),
                            r#schema_url: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl ResourceMetrics {
                    ///Return a reference to `resource` as an `Option`
                    #[inline]
                    pub fn r#resource(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&self.r#resource)
                    }
                    ///Return a mutable reference to `resource` as an `Option`
                    #[inline]
                    pub fn mut_resource(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&mut self.r#resource)
                    }
                    ///Set the value and presence of `resource`
                    #[inline]
                    pub fn set_resource(
                        &mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) {
                        self._has.set_resource();
                        self.r#resource = value.into();
                    }
                    ///Clear the presence of `resource`
                    #[inline]
                    pub fn clear_resource(&mut self) {
                        self._has.clear_resource();
                    }
                }
                impl ::micropb::MessageDecode for ResourceMetrics {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#resource;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_resource();
                                }
                                2u32 => {
                                    let mut val: ScopeMetrics = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#scope_metrics.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#schema_url;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ResourceMetrics {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            if let Some(val_ref) = self.r#resource() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for val_ref in self.r#scope_metrics.iter() {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            if let Some(val_ref) = self.r#resource() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for val_ref in self.r#scope_metrics.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                pub mod ScopeMetrics_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `scope`
                        #[inline]
                        pub fn r#scope(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `scope`
                        #[inline]
                        pub fn set_scope(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `scope`
                        #[inline]
                        pub fn clear_scope(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `scope`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_scope(mut self) -> Self {
                            self.set_scope();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ScopeMetrics {
                    pub r#scope: super::super::common_::v1_::InstrumentationScope,
                    pub r#metrics: crate::bumpalo::UnsafeVec<Metric>,
                    pub r#schema_url: crate::bumpalo::UnsafeString,
                    pub _has: ScopeMetrics_::_Hazzer,
                }
                impl ::core::default::Default for ScopeMetrics {
                    fn default() -> Self {
                        Self {
                            r#scope: ::core::default::Default::default(),
                            r#metrics: ::core::default::Default::default(),
                            r#schema_url: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl ScopeMetrics {
                    ///Return a reference to `scope` as an `Option`
                    #[inline]
                    pub fn r#scope(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&self.r#scope)
                    }
                    ///Return a mutable reference to `scope` as an `Option`
                    #[inline]
                    pub fn mut_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&mut self.r#scope)
                    }
                    ///Set the value and presence of `scope`
                    #[inline]
                    pub fn set_scope(
                        &mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) {
                        self._has.set_scope();
                        self.r#scope = value.into();
                    }
                    ///Clear the presence of `scope`
                    #[inline]
                    pub fn clear_scope(&mut self) {
                        self._has.clear_scope();
                    }
                }
                impl ::micropb::MessageDecode for ScopeMetrics {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#scope;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_scope();
                                }
                                2u32 => {
                                    let mut val: Metric = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#metrics.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#schema_url;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ScopeMetrics {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            if let Some(val_ref) = self.r#scope() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for val_ref in self.r#metrics.iter() {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            if let Some(val_ref) = self.r#scope() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for val_ref in self.r#metrics.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                pub mod Metric_ {
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Data {
                        Gauge(super::Gauge),
                        Sum(super::Sum),
                        Histogram(super::Histogram),
                        ExponentialHistogram(super::ExponentialHistogram),
                        Summary(super::Summary),
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Metric {
                    pub r#name: crate::bumpalo::UnsafeString,
                    pub r#description: crate::bumpalo::UnsafeString,
                    pub r#unit: crate::bumpalo::UnsafeString,
                    pub r#metadata: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#data: ::core::option::Option<Metric_::Data>,
                }
                impl ::core::default::Default for Metric {
                    fn default() -> Self {
                        Self {
                            r#name: ::core::default::Default::default(),
                            r#description: ::core::default::Default::default(),
                            r#unit: ::core::default::Default::default(),
                            r#metadata: ::core::default::Default::default(),
                            r#data: ::core::default::Default::default(),
                        }
                    }
                }
                impl Metric {}
                impl ::micropb::MessageDecode for Metric {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#name;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#description;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#unit;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                12u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#metadata.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                5u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#data
                                        {
                                            if let Metric_::Data::Gauge(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#data = ::core::option::Option::Some(
                                            Metric_::Data::Gauge(::core::default::Default::default()),
                                        );
                                    };
                                    mut_ref.decode_len_delimited(decoder)?;
                                }
                                7u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#data
                                        {
                                            if let Metric_::Data::Sum(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#data = ::core::option::Option::Some(
                                            Metric_::Data::Sum(::core::default::Default::default()),
                                        );
                                    };
                                    mut_ref.decode_len_delimited(decoder)?;
                                }
                                9u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#data
                                        {
                                            if let Metric_::Data::Histogram(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#data = ::core::option::Option::Some(
                                            Metric_::Data::Histogram(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    mut_ref.decode_len_delimited(decoder)?;
                                }
                                10u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#data
                                        {
                                            if let Metric_::Data::ExponentialHistogram(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#data = ::core::option::Option::Some(
                                            Metric_::Data::ExponentialHistogram(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    mut_ref.decode_len_delimited(decoder)?;
                                }
                                11u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#data
                                        {
                                            if let Metric_::Data::Summary(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#data = ::core::option::Option::Some(
                                            Metric_::Data::Summary(::core::default::Default::default()),
                                        );
                                    };
                                    mut_ref.decode_len_delimited(decoder)?;
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Metric {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            let val_ref = &self.r#name;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(10u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#description;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(18u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#unit;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            for val_ref in self.r#metadata.iter() {
                                encoder.encode_varint32(98u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        if let Some(oneof) = &self.r#data {
                            match &*oneof {
                                Metric_::Data::Gauge(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(42u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                                Metric_::Data::Sum(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(58u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                                Metric_::Data::Histogram(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(74u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                                Metric_::Data::ExponentialHistogram(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(82u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                                Metric_::Data::Summary(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(90u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            let val_ref = &self.r#name;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#description;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#unit;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            for val_ref in self.r#metadata.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        if let Some(oneof) = &self.r#data {
                            match &*oneof {
                                Metric_::Data::Gauge(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                                Metric_::Data::Sum(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                                Metric_::Data::Histogram(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                                Metric_::Data::ExponentialHistogram(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                                Metric_::Data::Summary(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Gauge {
                    pub r#data_points: crate::bumpalo::UnsafeVec<NumberDataPoint>,
                }
                impl ::core::default::Default for Gauge {
                    fn default() -> Self {
                        Self {
                            r#data_points: ::core::default::Default::default(),
                        }
                    }
                }
                impl Gauge {}
                impl ::micropb::MessageDecode for Gauge {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: NumberDataPoint = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#data_points.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Gauge {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#data_points.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#data_points.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Sum {
                    pub r#data_points: crate::bumpalo::UnsafeVec<NumberDataPoint>,
                    pub r#aggregation_temporality: AggregationTemporality,
                    pub r#is_monotonic: bool,
                }
                impl ::core::default::Default for Sum {
                    fn default() -> Self {
                        Self {
                            r#data_points: ::core::default::Default::default(),
                            r#aggregation_temporality: ::core::default::Default::default(),
                            r#is_monotonic: ::core::default::Default::default(),
                        }
                    }
                }
                impl Sum {}
                impl ::micropb::MessageDecode for Sum {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: NumberDataPoint = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#data_points.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#aggregation_temporality;
                                    {
                                        let val = decoder
                                            .decode_int32()
                                            .map(|n| AggregationTemporality(n as _))?;
                                        let val_ref = &val;
                                        if val_ref.0 != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#is_monotonic;
                                    {
                                        let val = decoder.decode_bool()?;
                                        let val_ref = &val;
                                        if *val_ref {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Sum {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#data_points.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#aggregation_temporality;
                            if val_ref.0 != 0 {
                                encoder.encode_varint32(16u32)?;
                                encoder.encode_int32(val_ref.0 as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#is_monotonic;
                            if *val_ref {
                                encoder.encode_varint32(24u32)?;
                                encoder.encode_bool(*val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#data_points.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#aggregation_temporality;
                            if val_ref.0 != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
                            }
                        }
                        {
                            let val_ref = &self.r#is_monotonic;
                            if *val_ref {
                                size += 1usize + 1;
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Histogram {
                    pub r#data_points: crate::bumpalo::UnsafeVec<HistogramDataPoint>,
                    pub r#aggregation_temporality: AggregationTemporality,
                }
                impl ::core::default::Default for Histogram {
                    fn default() -> Self {
                        Self {
                            r#data_points: ::core::default::Default::default(),
                            r#aggregation_temporality: ::core::default::Default::default(),
                        }
                    }
                }
                impl Histogram {}
                impl ::micropb::MessageDecode for Histogram {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: HistogramDataPoint = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#data_points.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#aggregation_temporality;
                                    {
                                        let val = decoder
                                            .decode_int32()
                                            .map(|n| AggregationTemporality(n as _))?;
                                        let val_ref = &val;
                                        if val_ref.0 != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Histogram {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#data_points.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#aggregation_temporality;
                            if val_ref.0 != 0 {
                                encoder.encode_varint32(16u32)?;
                                encoder.encode_int32(val_ref.0 as _)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#data_points.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#aggregation_temporality;
                            if val_ref.0 != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ExponentialHistogram {
                    pub r#data_points: crate::bumpalo::UnsafeVec<
                        ExponentialHistogramDataPoint,
                    >,
                    pub r#aggregation_temporality: AggregationTemporality,
                }
                impl ::core::default::Default for ExponentialHistogram {
                    fn default() -> Self {
                        Self {
                            r#data_points: ::core::default::Default::default(),
                            r#aggregation_temporality: ::core::default::Default::default(),
                        }
                    }
                }
                impl ExponentialHistogram {}
                impl ::micropb::MessageDecode for ExponentialHistogram {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: ExponentialHistogramDataPoint = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#data_points.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#aggregation_temporality;
                                    {
                                        let val = decoder
                                            .decode_int32()
                                            .map(|n| AggregationTemporality(n as _))?;
                                        let val_ref = &val;
                                        if val_ref.0 != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ExponentialHistogram {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#data_points.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#aggregation_temporality;
                            if val_ref.0 != 0 {
                                encoder.encode_varint32(16u32)?;
                                encoder.encode_int32(val_ref.0 as _)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#data_points.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#aggregation_temporality;
                            if val_ref.0 != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Summary {
                    pub r#data_points: crate::bumpalo::UnsafeVec<SummaryDataPoint>,
                }
                impl ::core::default::Default for Summary {
                    fn default() -> Self {
                        Self {
                            r#data_points: ::core::default::Default::default(),
                        }
                    }
                }
                impl Summary {}
                impl ::micropb::MessageDecode for Summary {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: SummaryDataPoint = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#data_points.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Summary {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#data_points.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#data_points.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                pub mod NumberDataPoint_ {
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Value {
                        AsDouble(f64),
                        AsInt(i64),
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct NumberDataPoint {
                    pub r#attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#start_time_unix_nano: u64,
                    pub r#time_unix_nano: u64,
                    pub r#exemplars: crate::bumpalo::UnsafeVec<Exemplar>,
                    pub r#flags: u32,
                    pub r#value: ::core::option::Option<NumberDataPoint_::Value>,
                }
                impl ::core::default::Default for NumberDataPoint {
                    fn default() -> Self {
                        Self {
                            r#attributes: ::core::default::Default::default(),
                            r#start_time_unix_nano: ::core::default::Default::default(),
                            r#time_unix_nano: ::core::default::Default::default(),
                            r#exemplars: ::core::default::Default::default(),
                            r#flags: ::core::default::Default::default(),
                            r#value: ::core::default::Default::default(),
                        }
                    }
                }
                impl NumberDataPoint {}
                impl ::micropb::MessageDecode for NumberDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                7u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#start_time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                5u32 => {
                                    let mut val: Exemplar = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#exemplars.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                8u32 => {
                                    let mut_ref = &mut self.r#flags;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                4u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let NumberDataPoint_::Value::AsDouble(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            NumberDataPoint_::Value::AsDouble(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    let val = decoder.decode_double()?;
                                    *mut_ref = val as _;
                                }
                                6u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let NumberDataPoint_::Value::AsInt(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            NumberDataPoint_::Value::AsInt(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    let val = decoder.decode_sfixed64()?;
                                    *mut_ref = val as _;
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for NumberDataPoint {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(58u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(17u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(25u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            for val_ref in self.r#exemplars.iter() {
                                encoder.encode_varint32(42u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                encoder.encode_varint32(64u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        if let Some(oneof) = &self.r#value {
                            match &*oneof {
                                NumberDataPoint_::Value::AsDouble(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(33u32)?;
                                    encoder.encode_double(*val_ref)?;
                                }
                                NumberDataPoint_::Value::AsInt(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(49u32)?;
                                    encoder.encode_sfixed64(*val_ref as _)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            for val_ref in self.r#exemplars.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        if let Some(oneof) = &self.r#value {
                            match &*oneof {
                                NumberDataPoint_::Value::AsDouble(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size += 1usize + 8;
                                }
                                NumberDataPoint_::Value::AsInt(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size += 1usize + 8;
                                }
                            }
                        }
                        size
                    }
                }
                pub mod HistogramDataPoint_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `sum`
                        #[inline]
                        pub fn r#sum(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `sum`
                        #[inline]
                        pub fn set_sum(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `sum`
                        #[inline]
                        pub fn clear_sum(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `sum`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_sum(mut self) -> Self {
                            self.set_sum();
                            self
                        }
                        ///Query presence of `min`
                        #[inline]
                        pub fn r#min(&self) -> bool {
                            (self.0[0] & 2) != 0
                        }
                        ///Set presence of `min`
                        #[inline]
                        pub fn set_min(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 2;
                        }
                        ///Clear presence of `min`
                        #[inline]
                        pub fn clear_min(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !2;
                        }
                        ///Builder method that sets the presence of `min`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_min(mut self) -> Self {
                            self.set_min();
                            self
                        }
                        ///Query presence of `max`
                        #[inline]
                        pub fn r#max(&self) -> bool {
                            (self.0[0] & 4) != 0
                        }
                        ///Set presence of `max`
                        #[inline]
                        pub fn set_max(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 4;
                        }
                        ///Clear presence of `max`
                        #[inline]
                        pub fn clear_max(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !4;
                        }
                        ///Builder method that sets the presence of `max`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_max(mut self) -> Self {
                            self.set_max();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct HistogramDataPoint {
                    pub r#attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#start_time_unix_nano: u64,
                    pub r#time_unix_nano: u64,
                    pub r#count: u64,
                    pub r#sum: f64,
                    pub r#bucket_counts: crate::bumpalo::UnsafeVec<u64>,
                    pub r#explicit_bounds: crate::bumpalo::UnsafeVec<f64>,
                    pub r#exemplars: crate::bumpalo::UnsafeVec<Exemplar>,
                    pub r#flags: u32,
                    pub r#min: f64,
                    pub r#max: f64,
                    pub _has: HistogramDataPoint_::_Hazzer,
                }
                impl ::core::default::Default for HistogramDataPoint {
                    fn default() -> Self {
                        Self {
                            r#attributes: ::core::default::Default::default(),
                            r#start_time_unix_nano: ::core::default::Default::default(),
                            r#time_unix_nano: ::core::default::Default::default(),
                            r#count: ::core::default::Default::default(),
                            r#sum: ::core::default::Default::default(),
                            r#bucket_counts: ::core::default::Default::default(),
                            r#explicit_bounds: ::core::default::Default::default(),
                            r#exemplars: ::core::default::Default::default(),
                            r#flags: ::core::default::Default::default(),
                            r#min: ::core::default::Default::default(),
                            r#max: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl HistogramDataPoint {
                    ///Return a reference to `sum` as an `Option`
                    #[inline]
                    pub fn r#sum(&self) -> ::core::option::Option<&f64> {
                        self._has.r#sum().then_some(&self.r#sum)
                    }
                    ///Return a mutable reference to `sum` as an `Option`
                    #[inline]
                    pub fn mut_sum(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#sum().then_some(&mut self.r#sum)
                    }
                    ///Set the value and presence of `sum`
                    #[inline]
                    pub fn set_sum(&mut self, value: f64) {
                        self._has.set_sum();
                        self.r#sum = value.into();
                    }
                    ///Clear the presence of `sum`
                    #[inline]
                    pub fn clear_sum(&mut self) {
                        self._has.clear_sum();
                    }
                    ///Return a reference to `min` as an `Option`
                    #[inline]
                    pub fn r#min(&self) -> ::core::option::Option<&f64> {
                        self._has.r#min().then_some(&self.r#min)
                    }
                    ///Return a mutable reference to `min` as an `Option`
                    #[inline]
                    pub fn mut_min(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#min().then_some(&mut self.r#min)
                    }
                    ///Set the value and presence of `min`
                    #[inline]
                    pub fn set_min(&mut self, value: f64) {
                        self._has.set_min();
                        self.r#min = value.into();
                    }
                    ///Clear the presence of `min`
                    #[inline]
                    pub fn clear_min(&mut self) {
                        self._has.clear_min();
                    }
                    ///Return a reference to `max` as an `Option`
                    #[inline]
                    pub fn r#max(&self) -> ::core::option::Option<&f64> {
                        self._has.r#max().then_some(&self.r#max)
                    }
                    ///Return a mutable reference to `max` as an `Option`
                    #[inline]
                    pub fn mut_max(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#max().then_some(&mut self.r#max)
                    }
                    ///Set the value and presence of `max`
                    #[inline]
                    pub fn set_max(&mut self, value: f64) {
                        self._has.set_max();
                        self.r#max = value.into();
                    }
                    ///Clear the presence of `max`
                    #[inline]
                    pub fn clear_max(&mut self) {
                        self._has.clear_max();
                    }
                }
                impl ::micropb::MessageDecode for HistogramDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                9u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#start_time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                4u32 => {
                                    let mut_ref = &mut self.r#count;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                5u32 => {
                                    let mut_ref = &mut self.r#sum;
                                    {
                                        let val = decoder.decode_double()?;
                                        *mut_ref = val as _;
                                    };
                                    self._has.set_sum();
                                }
                                6u32 => {
                                    if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                        decoder
                                            .decode_packed(
                                                &mut self.r#bucket_counts,
                                                |decoder| decoder.decode_fixed64().map(|v| v as _),
                                            )?;
                                    } else {
                                        if let (Err(_), false) = (
                                            self
                                                .r#bucket_counts
                                                .pb_push(decoder.decode_fixed64()? as _),
                                            decoder.ignore_repeated_cap_err,
                                        ) {
                                            return Err(::micropb::DecodeError::Capacity);
                                        }
                                    }
                                }
                                7u32 => {
                                    if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                        decoder
                                            .decode_packed(
                                                &mut self.r#explicit_bounds,
                                                |decoder| decoder.decode_double().map(|v| v as _),
                                            )?;
                                    } else {
                                        if let (Err(_), false) = (
                                            self
                                                .r#explicit_bounds
                                                .pb_push(decoder.decode_double()? as _),
                                            decoder.ignore_repeated_cap_err,
                                        ) {
                                            return Err(::micropb::DecodeError::Capacity);
                                        }
                                    }
                                }
                                8u32 => {
                                    let mut val: Exemplar = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#exemplars.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                10u32 => {
                                    let mut_ref = &mut self.r#flags;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                11u32 => {
                                    let mut_ref = &mut self.r#min;
                                    {
                                        let val = decoder.decode_double()?;
                                        *mut_ref = val as _;
                                    };
                                    self._has.set_min();
                                }
                                12u32 => {
                                    let mut_ref = &mut self.r#max;
                                    {
                                        let val = decoder.decode_double()?;
                                        *mut_ref = val as _;
                                    };
                                    self._has.set_max();
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for HistogramDataPoint {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(74u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(17u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(25u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(33u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#sum() {
                                encoder.encode_varint32(41u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            for val_ref in self.r#bucket_counts.iter() {
                                encoder.encode_varint32(49u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            for val_ref in self.r#explicit_bounds.iter() {
                                encoder.encode_varint32(57u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            for val_ref in self.r#exemplars.iter() {
                                encoder.encode_varint32(66u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                encoder.encode_varint32(80u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#min() {
                                encoder.encode_varint32(89u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#max() {
                                encoder.encode_varint32(97u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#count;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#sum() {
                                size += 1usize + 8;
                            }
                        }
                        {
                            size += self.r#bucket_counts.len() * (1usize + 8usize);
                        }
                        {
                            size += self.r#explicit_bounds.len() * (1usize + 8usize);
                        }
                        {
                            for val_ref in self.r#exemplars.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#min() {
                                size += 1usize + 8;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#max() {
                                size += 1usize + 8;
                            }
                        }
                        size
                    }
                }
                pub mod ExponentialHistogramDataPoint_ {
                    #[derive(Debug, PartialEq, Clone)]
                    pub struct Buckets {
                        pub r#offset: i32,
                        pub r#bucket_counts: crate::bumpalo::UnsafeVec<u64>,
                    }
                    impl ::core::default::Default for Buckets {
                        fn default() -> Self {
                            Self {
                                r#offset: ::core::default::Default::default(),
                                r#bucket_counts: ::core::default::Default::default(),
                            }
                        }
                    }
                    impl Buckets {}
                    impl ::micropb::MessageDecode for Buckets {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#offset;
                                        {
                                            let val = decoder.decode_sint32()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    2u32 => {
                                        if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                            decoder
                                                .decode_packed(
                                                    &mut self.r#bucket_counts,
                                                    |decoder| decoder.decode_varint64().map(|v| v as _),
                                                )?;
                                        } else {
                                            if let (Err(_), false) = (
                                                self
                                                    .r#bucket_counts
                                                    .pb_push(decoder.decode_varint64()? as _),
                                                decoder.ignore_repeated_cap_err,
                                            ) {
                                                return Err(::micropb::DecodeError::Capacity);
                                            }
                                        }
                                    }
                                    _ => {
                                        decoder.skip_wire_value(tag.wire_type())?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    impl ::micropb::MessageEncode for Buckets {
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            {
                                let val_ref = &self.r#offset;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(8u32)?;
                                    encoder.encode_sint32(*val_ref as _)?;
                                }
                            }
                            {
                                for val_ref in self.r#bucket_counts.iter() {
                                    encoder.encode_varint32(16u32)?;
                                    encoder.encode_varint64(*val_ref as _)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#offset;
                                if *val_ref != 0 {
                                    size
                                        += 1usize + ::micropb::size::sizeof_sint32(*val_ref as _);
                                }
                            }
                            {
                                for val_ref in self.r#bucket_counts.iter() {
                                    size
                                        += 1usize + ::micropb::size::sizeof_varint64(*val_ref as _);
                                }
                            }
                            size
                        }
                    }
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `sum`
                        #[inline]
                        pub fn r#sum(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `sum`
                        #[inline]
                        pub fn set_sum(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `sum`
                        #[inline]
                        pub fn clear_sum(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `sum`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_sum(mut self) -> Self {
                            self.set_sum();
                            self
                        }
                        ///Query presence of `positive`
                        #[inline]
                        pub fn r#positive(&self) -> bool {
                            (self.0[0] & 2) != 0
                        }
                        ///Set presence of `positive`
                        #[inline]
                        pub fn set_positive(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 2;
                        }
                        ///Clear presence of `positive`
                        #[inline]
                        pub fn clear_positive(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !2;
                        }
                        ///Builder method that sets the presence of `positive`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_positive(mut self) -> Self {
                            self.set_positive();
                            self
                        }
                        ///Query presence of `negative`
                        #[inline]
                        pub fn r#negative(&self) -> bool {
                            (self.0[0] & 4) != 0
                        }
                        ///Set presence of `negative`
                        #[inline]
                        pub fn set_negative(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 4;
                        }
                        ///Clear presence of `negative`
                        #[inline]
                        pub fn clear_negative(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !4;
                        }
                        ///Builder method that sets the presence of `negative`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_negative(mut self) -> Self {
                            self.set_negative();
                            self
                        }
                        ///Query presence of `min`
                        #[inline]
                        pub fn r#min(&self) -> bool {
                            (self.0[0] & 8) != 0
                        }
                        ///Set presence of `min`
                        #[inline]
                        pub fn set_min(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 8;
                        }
                        ///Clear presence of `min`
                        #[inline]
                        pub fn clear_min(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !8;
                        }
                        ///Builder method that sets the presence of `min`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_min(mut self) -> Self {
                            self.set_min();
                            self
                        }
                        ///Query presence of `max`
                        #[inline]
                        pub fn r#max(&self) -> bool {
                            (self.0[0] & 16) != 0
                        }
                        ///Set presence of `max`
                        #[inline]
                        pub fn set_max(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 16;
                        }
                        ///Clear presence of `max`
                        #[inline]
                        pub fn clear_max(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !16;
                        }
                        ///Builder method that sets the presence of `max`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_max(mut self) -> Self {
                            self.set_max();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ExponentialHistogramDataPoint {
                    pub r#attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#start_time_unix_nano: u64,
                    pub r#time_unix_nano: u64,
                    pub r#count: u64,
                    pub r#sum: f64,
                    pub r#scale: i32,
                    pub r#zero_count: u64,
                    pub r#positive: ExponentialHistogramDataPoint_::Buckets,
                    pub r#negative: ExponentialHistogramDataPoint_::Buckets,
                    pub r#flags: u32,
                    pub r#exemplars: crate::bumpalo::UnsafeVec<Exemplar>,
                    pub r#min: f64,
                    pub r#max: f64,
                    pub r#zero_threshold: f64,
                    pub _has: ExponentialHistogramDataPoint_::_Hazzer,
                }
                impl ::core::default::Default for ExponentialHistogramDataPoint {
                    fn default() -> Self {
                        Self {
                            r#attributes: ::core::default::Default::default(),
                            r#start_time_unix_nano: ::core::default::Default::default(),
                            r#time_unix_nano: ::core::default::Default::default(),
                            r#count: ::core::default::Default::default(),
                            r#sum: ::core::default::Default::default(),
                            r#scale: ::core::default::Default::default(),
                            r#zero_count: ::core::default::Default::default(),
                            r#positive: ::core::default::Default::default(),
                            r#negative: ::core::default::Default::default(),
                            r#flags: ::core::default::Default::default(),
                            r#exemplars: ::core::default::Default::default(),
                            r#min: ::core::default::Default::default(),
                            r#max: ::core::default::Default::default(),
                            r#zero_threshold: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl ExponentialHistogramDataPoint {
                    ///Return a reference to `sum` as an `Option`
                    #[inline]
                    pub fn r#sum(&self) -> ::core::option::Option<&f64> {
                        self._has.r#sum().then_some(&self.r#sum)
                    }
                    ///Return a mutable reference to `sum` as an `Option`
                    #[inline]
                    pub fn mut_sum(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#sum().then_some(&mut self.r#sum)
                    }
                    ///Set the value and presence of `sum`
                    #[inline]
                    pub fn set_sum(&mut self, value: f64) {
                        self._has.set_sum();
                        self.r#sum = value.into();
                    }
                    ///Clear the presence of `sum`
                    #[inline]
                    pub fn clear_sum(&mut self) {
                        self._has.clear_sum();
                    }
                    ///Return a reference to `positive` as an `Option`
                    #[inline]
                    pub fn r#positive(
                        &self,
                    ) -> ::core::option::Option<
                        &ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#positive().then_some(&self.r#positive)
                    }
                    ///Return a mutable reference to `positive` as an `Option`
                    #[inline]
                    pub fn mut_positive(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#positive().then_some(&mut self.r#positive)
                    }
                    ///Set the value and presence of `positive`
                    #[inline]
                    pub fn set_positive(
                        &mut self,
                        value: ExponentialHistogramDataPoint_::Buckets,
                    ) {
                        self._has.set_positive();
                        self.r#positive = value.into();
                    }
                    ///Clear the presence of `positive`
                    #[inline]
                    pub fn clear_positive(&mut self) {
                        self._has.clear_positive();
                    }
                    ///Return a reference to `negative` as an `Option`
                    #[inline]
                    pub fn r#negative(
                        &self,
                    ) -> ::core::option::Option<
                        &ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#negative().then_some(&self.r#negative)
                    }
                    ///Return a mutable reference to `negative` as an `Option`
                    #[inline]
                    pub fn mut_negative(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#negative().then_some(&mut self.r#negative)
                    }
                    ///Set the value and presence of `negative`
                    #[inline]
                    pub fn set_negative(
                        &mut self,
                        value: ExponentialHistogramDataPoint_::Buckets,
                    ) {
                        self._has.set_negative();
                        self.r#negative = value.into();
                    }
                    ///Clear the presence of `negative`
                    #[inline]
                    pub fn clear_negative(&mut self) {
                        self._has.clear_negative();
                    }
                    ///Return a reference to `min` as an `Option`
                    #[inline]
                    pub fn r#min(&self) -> ::core::option::Option<&f64> {
                        self._has.r#min().then_some(&self.r#min)
                    }
                    ///Return a mutable reference to `min` as an `Option`
                    #[inline]
                    pub fn mut_min(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#min().then_some(&mut self.r#min)
                    }
                    ///Set the value and presence of `min`
                    #[inline]
                    pub fn set_min(&mut self, value: f64) {
                        self._has.set_min();
                        self.r#min = value.into();
                    }
                    ///Clear the presence of `min`
                    #[inline]
                    pub fn clear_min(&mut self) {
                        self._has.clear_min();
                    }
                    ///Return a reference to `max` as an `Option`
                    #[inline]
                    pub fn r#max(&self) -> ::core::option::Option<&f64> {
                        self._has.r#max().then_some(&self.r#max)
                    }
                    ///Return a mutable reference to `max` as an `Option`
                    #[inline]
                    pub fn mut_max(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#max().then_some(&mut self.r#max)
                    }
                    ///Set the value and presence of `max`
                    #[inline]
                    pub fn set_max(&mut self, value: f64) {
                        self._has.set_max();
                        self.r#max = value.into();
                    }
                    ///Clear the presence of `max`
                    #[inline]
                    pub fn clear_max(&mut self) {
                        self._has.clear_max();
                    }
                }
                impl ::micropb::MessageDecode for ExponentialHistogramDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#start_time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                4u32 => {
                                    let mut_ref = &mut self.r#count;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                5u32 => {
                                    let mut_ref = &mut self.r#sum;
                                    {
                                        let val = decoder.decode_double()?;
                                        *mut_ref = val as _;
                                    };
                                    self._has.set_sum();
                                }
                                6u32 => {
                                    let mut_ref = &mut self.r#scale;
                                    {
                                        let val = decoder.decode_sint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                7u32 => {
                                    let mut_ref = &mut self.r#zero_count;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                8u32 => {
                                    let mut_ref = &mut self.r#positive;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_positive();
                                }
                                9u32 => {
                                    let mut_ref = &mut self.r#negative;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_negative();
                                }
                                10u32 => {
                                    let mut_ref = &mut self.r#flags;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                11u32 => {
                                    let mut val: Exemplar = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#exemplars.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                12u32 => {
                                    let mut_ref = &mut self.r#min;
                                    {
                                        let val = decoder.decode_double()?;
                                        *mut_ref = val as _;
                                    };
                                    self._has.set_min();
                                }
                                13u32 => {
                                    let mut_ref = &mut self.r#max;
                                    {
                                        let val = decoder.decode_double()?;
                                        *mut_ref = val as _;
                                    };
                                    self._has.set_max();
                                }
                                14u32 => {
                                    let mut_ref = &mut self.r#zero_threshold;
                                    {
                                        let val = decoder.decode_double()?;
                                        let val_ref = &val;
                                        if *val_ref != 0.0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ExponentialHistogramDataPoint {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(17u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(25u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(33u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#sum() {
                                encoder.encode_varint32(41u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#scale;
                            if *val_ref != 0 {
                                encoder.encode_varint32(48u32)?;
                                encoder.encode_sint32(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#zero_count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(57u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#positive() {
                                encoder.encode_varint32(66u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#negative() {
                                encoder.encode_varint32(74u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                encoder.encode_varint32(80u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        {
                            for val_ref in self.r#exemplars.iter() {
                                encoder.encode_varint32(90u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#min() {
                                encoder.encode_varint32(97u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#max() {
                                encoder.encode_varint32(105u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#zero_threshold;
                            if *val_ref != 0.0 {
                                encoder.encode_varint32(113u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#count;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#sum() {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#scale;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_sint32(*val_ref as _);
                            }
                        }
                        {
                            let val_ref = &self.r#zero_count;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#positive() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#negative() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        {
                            for val_ref in self.r#exemplars.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#min() {
                                size += 1usize + 8;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#max() {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#zero_threshold;
                            if *val_ref != 0.0 {
                                size += 1usize + 8;
                            }
                        }
                        size
                    }
                }
                pub mod SummaryDataPoint_ {
                    #[derive(Debug, PartialEq, Clone)]
                    pub struct ValueAtQuantile {
                        pub r#quantile: f64,
                        pub r#value: f64,
                    }
                    impl ::core::default::Default for ValueAtQuantile {
                        fn default() -> Self {
                            Self {
                                r#quantile: ::core::default::Default::default(),
                                r#value: ::core::default::Default::default(),
                            }
                        }
                    }
                    impl ValueAtQuantile {}
                    impl ::micropb::MessageDecode for ValueAtQuantile {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#quantile;
                                        {
                                            let val = decoder.decode_double()?;
                                            let val_ref = &val;
                                            if *val_ref != 0.0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    2u32 => {
                                        let mut_ref = &mut self.r#value;
                                        {
                                            let val = decoder.decode_double()?;
                                            let val_ref = &val;
                                            if *val_ref != 0.0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    _ => {
                                        decoder.skip_wire_value(tag.wire_type())?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    impl ::micropb::MessageEncode for ValueAtQuantile {
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            {
                                let val_ref = &self.r#quantile;
                                if *val_ref != 0.0 {
                                    encoder.encode_varint32(9u32)?;
                                    encoder.encode_double(*val_ref)?;
                                }
                            }
                            {
                                let val_ref = &self.r#value;
                                if *val_ref != 0.0 {
                                    encoder.encode_varint32(17u32)?;
                                    encoder.encode_double(*val_ref)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#quantile;
                                if *val_ref != 0.0 {
                                    size += 1usize + 8;
                                }
                            }
                            {
                                let val_ref = &self.r#value;
                                if *val_ref != 0.0 {
                                    size += 1usize + 8;
                                }
                            }
                            size
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct SummaryDataPoint {
                    pub r#attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#start_time_unix_nano: u64,
                    pub r#time_unix_nano: u64,
                    pub r#count: u64,
                    pub r#sum: f64,
                    pub r#quantile_values: crate::bumpalo::UnsafeVec<
                        SummaryDataPoint_::ValueAtQuantile,
                    >,
                    pub r#flags: u32,
                }
                impl ::core::default::Default for SummaryDataPoint {
                    fn default() -> Self {
                        Self {
                            r#attributes: ::core::default::Default::default(),
                            r#start_time_unix_nano: ::core::default::Default::default(),
                            r#time_unix_nano: ::core::default::Default::default(),
                            r#count: ::core::default::Default::default(),
                            r#sum: ::core::default::Default::default(),
                            r#quantile_values: ::core::default::Default::default(),
                            r#flags: ::core::default::Default::default(),
                        }
                    }
                }
                impl SummaryDataPoint {}
                impl ::micropb::MessageDecode for SummaryDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                7u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#start_time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                4u32 => {
                                    let mut_ref = &mut self.r#count;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                5u32 => {
                                    let mut_ref = &mut self.r#sum;
                                    {
                                        let val = decoder.decode_double()?;
                                        let val_ref = &val;
                                        if *val_ref != 0.0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                6u32 => {
                                    let mut val: SummaryDataPoint_::ValueAtQuantile = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#quantile_values.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                8u32 => {
                                    let mut_ref = &mut self.r#flags;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for SummaryDataPoint {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(58u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(17u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(25u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(33u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#sum;
                            if *val_ref != 0.0 {
                                encoder.encode_varint32(41u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            for val_ref in self.r#quantile_values.iter() {
                                encoder.encode_varint32(50u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                encoder.encode_varint32(64u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#count;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#sum;
                            if *val_ref != 0.0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            for val_ref in self.r#quantile_values.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        size
                    }
                }
                pub mod Exemplar_ {
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Value {
                        AsDouble(f64),
                        AsInt(i64),
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Exemplar {
                    pub r#filtered_attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#time_unix_nano: u64,
                    pub r#span_id: crate::bumpalo::UnsafeVec<u8>,
                    pub r#trace_id: crate::bumpalo::UnsafeVec<u8>,
                    pub r#value: ::core::option::Option<Exemplar_::Value>,
                }
                impl ::core::default::Default for Exemplar {
                    fn default() -> Self {
                        Self {
                            r#filtered_attributes: ::core::default::Default::default(),
                            r#time_unix_nano: ::core::default::Default::default(),
                            r#span_id: ::core::default::Default::default(),
                            r#trace_id: ::core::default::Default::default(),
                            r#value: ::core::default::Default::default(),
                        }
                    }
                }
                impl Exemplar {}
                impl ::micropb::MessageDecode for Exemplar {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                7u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#filtered_attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                4u32 => {
                                    let mut_ref = &mut self.r#span_id;
                                    {
                                        decoder
                                            .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                5u32 => {
                                    let mut_ref = &mut self.r#trace_id;
                                    {
                                        decoder
                                            .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                3u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let Exemplar_::Value::AsDouble(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            Exemplar_::Value::AsDouble(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    let val = decoder.decode_double()?;
                                    *mut_ref = val as _;
                                }
                                6u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let Exemplar_::Value::AsInt(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            Exemplar_::Value::AsInt(::core::default::Default::default()),
                                        );
                                    };
                                    let val = decoder.decode_sfixed64()?;
                                    *mut_ref = val as _;
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Exemplar {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#filtered_attributes.iter() {
                                encoder.encode_varint32(58u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(17u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#span_id;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(34u32)?;
                                encoder.encode_bytes(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#trace_id;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(42u32)?;
                                encoder.encode_bytes(val_ref)?;
                            }
                        }
                        if let Some(oneof) = &self.r#value {
                            match &*oneof {
                                Exemplar_::Value::AsDouble(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(25u32)?;
                                    encoder.encode_double(*val_ref)?;
                                }
                                Exemplar_::Value::AsInt(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(49u32)?;
                                    encoder.encode_sfixed64(*val_ref as _)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#filtered_attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#span_id;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#trace_id;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        if let Some(oneof) = &self.r#value {
                            match &*oneof {
                                Exemplar_::Value::AsDouble(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size += 1usize + 8;
                                }
                                Exemplar_::Value::AsInt(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size += 1usize + 8;
                                }
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct AggregationTemporality(pub i32);
                impl AggregationTemporality {
                    pub const Unspecified: Self = Self(0);
                    pub const Delta: Self = Self(1);
                    pub const Cumulative: Self = Self(2);
                }
                impl core::default::Default for AggregationTemporality {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl core::convert::From<i32> for AggregationTemporality {
                    fn from(val: i32) -> Self {
                        Self(val)
                    }
                }
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct DataPointFlags(pub i32);
                impl DataPointFlags {
                    pub const DoNotUse: Self = Self(0);
                    pub const NoRecordedValueMask: Self = Self(1);
                }
                impl core::default::Default for DataPointFlags {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl core::convert::From<i32> for DataPointFlags {
                    fn from(val: i32) -> Self {
                        Self(val)
                    }
                }
            }
        }
        pub mod trace_ {
            pub mod v1_ {
                #[derive(Debug, PartialEq, Clone)]
                pub struct TracesData {
                    pub r#resource_spans: crate::bumpalo::UnsafeVec<ResourceSpans>,
                }
                impl ::core::default::Default for TracesData {
                    fn default() -> Self {
                        Self {
                            r#resource_spans: ::core::default::Default::default(),
                        }
                    }
                }
                impl TracesData {}
                impl ::micropb::MessageDecode for TracesData {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut val: ResourceSpans = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#resource_spans.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for TracesData {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            for val_ref in self.r#resource_spans.iter() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            for val_ref in self.r#resource_spans.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                pub mod ResourceSpans_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `resource`
                        #[inline]
                        pub fn r#resource(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `resource`
                        #[inline]
                        pub fn set_resource(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `resource`
                        #[inline]
                        pub fn clear_resource(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `resource`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_resource(mut self) -> Self {
                            self.set_resource();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ResourceSpans {
                    pub r#resource: super::super::resource_::v1_::Resource,
                    pub r#scope_spans: crate::bumpalo::UnsafeVec<ScopeSpans>,
                    pub r#schema_url: crate::bumpalo::UnsafeString,
                    pub _has: ResourceSpans_::_Hazzer,
                }
                impl ::core::default::Default for ResourceSpans {
                    fn default() -> Self {
                        Self {
                            r#resource: ::core::default::Default::default(),
                            r#scope_spans: ::core::default::Default::default(),
                            r#schema_url: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl ResourceSpans {
                    ///Return a reference to `resource` as an `Option`
                    #[inline]
                    pub fn r#resource(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&self.r#resource)
                    }
                    ///Return a mutable reference to `resource` as an `Option`
                    #[inline]
                    pub fn mut_resource(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&mut self.r#resource)
                    }
                    ///Set the value and presence of `resource`
                    #[inline]
                    pub fn set_resource(
                        &mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) {
                        self._has.set_resource();
                        self.r#resource = value.into();
                    }
                    ///Clear the presence of `resource`
                    #[inline]
                    pub fn clear_resource(&mut self) {
                        self._has.clear_resource();
                    }
                }
                impl ::micropb::MessageDecode for ResourceSpans {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#resource;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_resource();
                                }
                                2u32 => {
                                    let mut val: ScopeSpans = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#scope_spans.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#schema_url;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ResourceSpans {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            if let Some(val_ref) = self.r#resource() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for val_ref in self.r#scope_spans.iter() {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            if let Some(val_ref) = self.r#resource() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for val_ref in self.r#scope_spans.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                pub mod ScopeSpans_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `scope`
                        #[inline]
                        pub fn r#scope(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `scope`
                        #[inline]
                        pub fn set_scope(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `scope`
                        #[inline]
                        pub fn clear_scope(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `scope`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_scope(mut self) -> Self {
                            self.set_scope();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct ScopeSpans {
                    pub r#scope: super::super::common_::v1_::InstrumentationScope,
                    pub r#spans: crate::bumpalo::UnsafeVec<Span>,
                    pub r#schema_url: crate::bumpalo::UnsafeString,
                    pub _has: ScopeSpans_::_Hazzer,
                }
                impl ::core::default::Default for ScopeSpans {
                    fn default() -> Self {
                        Self {
                            r#scope: ::core::default::Default::default(),
                            r#spans: ::core::default::Default::default(),
                            r#schema_url: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl ScopeSpans {
                    ///Return a reference to `scope` as an `Option`
                    #[inline]
                    pub fn r#scope(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&self.r#scope)
                    }
                    ///Return a mutable reference to `scope` as an `Option`
                    #[inline]
                    pub fn mut_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&mut self.r#scope)
                    }
                    ///Set the value and presence of `scope`
                    #[inline]
                    pub fn set_scope(
                        &mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) {
                        self._has.set_scope();
                        self.r#scope = value.into();
                    }
                    ///Clear the presence of `scope`
                    #[inline]
                    pub fn clear_scope(&mut self) {
                        self._has.clear_scope();
                    }
                }
                impl ::micropb::MessageDecode for ScopeSpans {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#scope;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_scope();
                                }
                                2u32 => {
                                    let mut val: Span = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#spans.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#schema_url;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for ScopeSpans {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            if let Some(val_ref) = self.r#scope() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for val_ref in self.r#spans.iter() {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            if let Some(val_ref) = self.r#scope() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for val_ref in self.r#spans.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                pub mod Span_ {
                    #[derive(Debug, PartialEq, Clone)]
                    pub struct Event {
                        pub r#time_unix_nano: u64,
                        pub r#name: crate::bumpalo::UnsafeString,
                        pub r#attributes: crate::bumpalo::UnsafeVec<
                            super::super::super::common_::v1_::KeyValue,
                        >,
                        pub r#dropped_attributes_count: u32,
                    }
                    impl ::core::default::Default for Event {
                        fn default() -> Self {
                            Self {
                                r#time_unix_nano: ::core::default::Default::default(),
                                r#name: ::core::default::Default::default(),
                                r#attributes: ::core::default::Default::default(),
                                r#dropped_attributes_count: ::core::default::Default::default(),
                            }
                        }
                    }
                    impl Event {}
                    impl ::micropb::MessageDecode for Event {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#time_unix_nano;
                                        {
                                            let val = decoder.decode_fixed64()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    2u32 => {
                                        let mut_ref = &mut self.r#name;
                                        {
                                            decoder
                                                .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                        };
                                    }
                                    3u32 => {
                                        let mut val: super::super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                        let mut_ref = &mut val;
                                        {
                                            mut_ref.decode_len_delimited(decoder)?;
                                        };
                                        if let (Err(_), false) = (
                                            self.r#attributes.pb_push(val),
                                            decoder.ignore_repeated_cap_err,
                                        ) {
                                            return Err(::micropb::DecodeError::Capacity);
                                        }
                                    }
                                    4u32 => {
                                        let mut_ref = &mut self.r#dropped_attributes_count;
                                        {
                                            let val = decoder.decode_varint32()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    _ => {
                                        decoder.skip_wire_value(tag.wire_type())?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    impl ::micropb::MessageEncode for Event {
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            {
                                let val_ref = &self.r#time_unix_nano;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(9u32)?;
                                    encoder.encode_fixed64(*val_ref as _)?;
                                }
                            }
                            {
                                let val_ref = &self.r#name;
                                if !val_ref.is_empty() {
                                    encoder.encode_varint32(18u32)?;
                                    encoder.encode_string(val_ref)?;
                                }
                            }
                            {
                                for val_ref in self.r#attributes.iter() {
                                    encoder.encode_varint32(26u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            {
                                let val_ref = &self.r#dropped_attributes_count;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(32u32)?;
                                    encoder.encode_varint32(*val_ref as _)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#time_unix_nano;
                                if *val_ref != 0 {
                                    size += 1usize + 8;
                                }
                            }
                            {
                                let val_ref = &self.r#name;
                                if !val_ref.is_empty() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                            {
                                for val_ref in self.r#attributes.iter() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                            }
                            {
                                let val_ref = &self.r#dropped_attributes_count;
                                if *val_ref != 0 {
                                    size
                                        += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                                }
                            }
                            size
                        }
                    }
                    #[derive(Debug, PartialEq, Clone)]
                    pub struct Link {
                        pub r#trace_id: crate::bumpalo::UnsafeVec<u8>,
                        pub r#span_id: crate::bumpalo::UnsafeVec<u8>,
                        pub r#trace_state: crate::bumpalo::UnsafeString,
                        pub r#attributes: crate::bumpalo::UnsafeVec<
                            super::super::super::common_::v1_::KeyValue,
                        >,
                        pub r#dropped_attributes_count: u32,
                        pub r#flags: u32,
                    }
                    impl ::core::default::Default for Link {
                        fn default() -> Self {
                            Self {
                                r#trace_id: ::core::default::Default::default(),
                                r#span_id: ::core::default::Default::default(),
                                r#trace_state: ::core::default::Default::default(),
                                r#attributes: ::core::default::Default::default(),
                                r#dropped_attributes_count: ::core::default::Default::default(),
                                r#flags: ::core::default::Default::default(),
                            }
                        }
                    }
                    impl Link {}
                    impl ::micropb::MessageDecode for Link {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#trace_id;
                                        {
                                            decoder
                                                .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                        };
                                    }
                                    2u32 => {
                                        let mut_ref = &mut self.r#span_id;
                                        {
                                            decoder
                                                .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                        };
                                    }
                                    3u32 => {
                                        let mut_ref = &mut self.r#trace_state;
                                        {
                                            decoder
                                                .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                        };
                                    }
                                    4u32 => {
                                        let mut val: super::super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                        let mut_ref = &mut val;
                                        {
                                            mut_ref.decode_len_delimited(decoder)?;
                                        };
                                        if let (Err(_), false) = (
                                            self.r#attributes.pb_push(val),
                                            decoder.ignore_repeated_cap_err,
                                        ) {
                                            return Err(::micropb::DecodeError::Capacity);
                                        }
                                    }
                                    5u32 => {
                                        let mut_ref = &mut self.r#dropped_attributes_count;
                                        {
                                            let val = decoder.decode_varint32()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    6u32 => {
                                        let mut_ref = &mut self.r#flags;
                                        {
                                            let val = decoder.decode_fixed32()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    _ => {
                                        decoder.skip_wire_value(tag.wire_type())?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    impl ::micropb::MessageEncode for Link {
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            {
                                let val_ref = &self.r#trace_id;
                                if !val_ref.is_empty() {
                                    encoder.encode_varint32(10u32)?;
                                    encoder.encode_bytes(val_ref)?;
                                }
                            }
                            {
                                let val_ref = &self.r#span_id;
                                if !val_ref.is_empty() {
                                    encoder.encode_varint32(18u32)?;
                                    encoder.encode_bytes(val_ref)?;
                                }
                            }
                            {
                                let val_ref = &self.r#trace_state;
                                if !val_ref.is_empty() {
                                    encoder.encode_varint32(26u32)?;
                                    encoder.encode_string(val_ref)?;
                                }
                            }
                            {
                                for val_ref in self.r#attributes.iter() {
                                    encoder.encode_varint32(34u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            {
                                let val_ref = &self.r#dropped_attributes_count;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(40u32)?;
                                    encoder.encode_varint32(*val_ref as _)?;
                                }
                            }
                            {
                                let val_ref = &self.r#flags;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(53u32)?;
                                    encoder.encode_fixed32(*val_ref as _)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#trace_id;
                                if !val_ref.is_empty() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                            {
                                let val_ref = &self.r#span_id;
                                if !val_ref.is_empty() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                            {
                                let val_ref = &self.r#trace_state;
                                if !val_ref.is_empty() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                            {
                                for val_ref in self.r#attributes.iter() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(
                                                val_ref.compute_size(),
                                            );
                                }
                            }
                            {
                                let val_ref = &self.r#dropped_attributes_count;
                                if *val_ref != 0 {
                                    size
                                        += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                                }
                            }
                            {
                                let val_ref = &self.r#flags;
                                if *val_ref != 0 {
                                    size += 1usize + 4;
                                }
                            }
                            size
                        }
                    }
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[repr(transparent)]
                    pub struct SpanKind(pub i32);
                    impl SpanKind {
                        pub const Unspecified: Self = Self(0);
                        pub const Internal: Self = Self(1);
                        pub const Server: Self = Self(2);
                        pub const Client: Self = Self(3);
                        pub const Producer: Self = Self(4);
                        pub const Consumer: Self = Self(5);
                    }
                    impl core::default::Default for SpanKind {
                        fn default() -> Self {
                            Self(0)
                        }
                    }
                    impl core::convert::From<i32> for SpanKind {
                        fn from(val: i32) -> Self {
                            Self(val)
                        }
                    }
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        ///Query presence of `status`
                        #[inline]
                        pub fn r#status(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        ///Set presence of `status`
                        #[inline]
                        pub fn set_status(&mut self) {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                        }
                        ///Clear presence of `status`
                        #[inline]
                        pub fn clear_status(&mut self) {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                        }
                        ///Builder method that sets the presence of `status`. Useful for initializing the Hazzer.
                        #[inline]
                        pub fn init_status(mut self) -> Self {
                            self.set_status();
                            self
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Span {
                    pub r#trace_id: crate::bumpalo::UnsafeVec<u8>,
                    pub r#span_id: crate::bumpalo::UnsafeVec<u8>,
                    pub r#trace_state: crate::bumpalo::UnsafeString,
                    pub r#parent_span_id: crate::bumpalo::UnsafeVec<u8>,
                    pub r#flags: u32,
                    pub r#name: crate::bumpalo::UnsafeString,
                    pub r#kind: Span_::SpanKind,
                    pub r#start_time_unix_nano: u64,
                    pub r#end_time_unix_nano: u64,
                    pub r#attributes: crate::bumpalo::UnsafeVec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#dropped_attributes_count: u32,
                    pub r#events: crate::bumpalo::UnsafeVec<Span_::Event>,
                    pub r#dropped_events_count: u32,
                    pub r#links: crate::bumpalo::UnsafeVec<Span_::Link>,
                    pub r#dropped_links_count: u32,
                    pub r#status: Status,
                    pub _has: Span_::_Hazzer,
                }
                impl ::core::default::Default for Span {
                    fn default() -> Self {
                        Self {
                            r#trace_id: ::core::default::Default::default(),
                            r#span_id: ::core::default::Default::default(),
                            r#trace_state: ::core::default::Default::default(),
                            r#parent_span_id: ::core::default::Default::default(),
                            r#flags: ::core::default::Default::default(),
                            r#name: ::core::default::Default::default(),
                            r#kind: ::core::default::Default::default(),
                            r#start_time_unix_nano: ::core::default::Default::default(),
                            r#end_time_unix_nano: ::core::default::Default::default(),
                            r#attributes: ::core::default::Default::default(),
                            r#dropped_attributes_count: ::core::default::Default::default(),
                            r#events: ::core::default::Default::default(),
                            r#dropped_events_count: ::core::default::Default::default(),
                            r#links: ::core::default::Default::default(),
                            r#dropped_links_count: ::core::default::Default::default(),
                            r#status: ::core::default::Default::default(),
                            _has: ::core::default::Default::default(),
                        }
                    }
                }
                impl Span {
                    ///Return a reference to `status` as an `Option`
                    #[inline]
                    pub fn r#status(&self) -> ::core::option::Option<&Status> {
                        self._has.r#status().then_some(&self.r#status)
                    }
                    ///Return a mutable reference to `status` as an `Option`
                    #[inline]
                    pub fn mut_status(&mut self) -> ::core::option::Option<&mut Status> {
                        self._has.r#status().then_some(&mut self.r#status)
                    }
                    ///Set the value and presence of `status`
                    #[inline]
                    pub fn set_status(&mut self, value: Status) {
                        self._has.set_status();
                        self.r#status = value.into();
                    }
                    ///Clear the presence of `status`
                    #[inline]
                    pub fn clear_status(&mut self) {
                        self._has.clear_status();
                    }
                }
                impl ::micropb::MessageDecode for Span {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#trace_id;
                                    {
                                        decoder
                                            .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#span_id;
                                    {
                                        decoder
                                            .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#trace_state;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                4u32 => {
                                    let mut_ref = &mut self.r#parent_span_id;
                                    {
                                        decoder
                                            .decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                16u32 => {
                                    let mut_ref = &mut self.r#flags;
                                    {
                                        let val = decoder.decode_fixed32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                5u32 => {
                                    let mut_ref = &mut self.r#name;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                6u32 => {
                                    let mut_ref = &mut self.r#kind;
                                    {
                                        let val = decoder
                                            .decode_int32()
                                            .map(|n| Span_::SpanKind(n as _))?;
                                        let val_ref = &val;
                                        if val_ref.0 != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                7u32 => {
                                    let mut_ref = &mut self.r#start_time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                8u32 => {
                                    let mut_ref = &mut self.r#end_time_unix_nano;
                                    {
                                        let val = decoder.decode_fixed64()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                9u32 => {
                                    let mut val: super::super::common_::v1_::KeyValue = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#attributes.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                10u32 => {
                                    let mut_ref = &mut self.r#dropped_attributes_count;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                11u32 => {
                                    let mut val: Span_::Event = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#events.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                12u32 => {
                                    let mut_ref = &mut self.r#dropped_events_count;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                13u32 => {
                                    let mut val: Span_::Link = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#links.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                14u32 => {
                                    let mut_ref = &mut self.r#dropped_links_count;
                                    {
                                        let val = decoder.decode_varint32()?;
                                        let val_ref = &val;
                                        if *val_ref != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                15u32 => {
                                    let mut_ref = &mut self.r#status;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    self._has.set_status();
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Span {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            let val_ref = &self.r#trace_id;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(10u32)?;
                                encoder.encode_bytes(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#span_id;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(18u32)?;
                                encoder.encode_bytes(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#trace_state;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#parent_span_id;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(34u32)?;
                                encoder.encode_bytes(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                encoder.encode_varint32(133u32)?;
                                encoder.encode_fixed32(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#name;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(42u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#kind;
                            if val_ref.0 != 0 {
                                encoder.encode_varint32(48u32)?;
                                encoder.encode_int32(val_ref.0 as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(57u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            let val_ref = &self.r#end_time_unix_nano;
                            if *val_ref != 0 {
                                encoder.encode_varint32(65u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            for val_ref in self.r#attributes.iter() {
                                encoder.encode_varint32(74u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(80u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        {
                            for val_ref in self.r#events.iter() {
                                encoder.encode_varint32(90u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_events_count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(96u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        {
                            for val_ref in self.r#links.iter() {
                                encoder.encode_varint32(106u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_links_count;
                            if *val_ref != 0 {
                                encoder.encode_varint32(112u32)?;
                                encoder.encode_varint32(*val_ref as _)?;
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#status() {
                                encoder.encode_varint32(122u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            let val_ref = &self.r#trace_id;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#span_id;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#trace_state;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#parent_span_id;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#flags;
                            if *val_ref != 0 {
                                size += 2usize + 4;
                            }
                        }
                        {
                            let val_ref = &self.r#name;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#kind;
                            if val_ref.0 != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
                            }
                        }
                        {
                            let val_ref = &self.r#start_time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            let val_ref = &self.r#end_time_unix_nano;
                            if *val_ref != 0 {
                                size += 1usize + 8;
                            }
                        }
                        {
                            for val_ref in self.r#attributes.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_attributes_count;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        {
                            for val_ref in self.r#events.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_events_count;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        {
                            for val_ref in self.r#links.iter() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#dropped_links_count;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                            }
                        }
                        {
                            if let Some(val_ref) = self.r#status() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        size
                    }
                }
                pub mod Status_ {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[repr(transparent)]
                    pub struct StatusCode(pub i32);
                    impl StatusCode {
                        pub const Unset: Self = Self(0);
                        pub const Ok: Self = Self(1);
                        pub const Error: Self = Self(2);
                    }
                    impl core::default::Default for StatusCode {
                        fn default() -> Self {
                            Self(0)
                        }
                    }
                    impl core::convert::From<i32> for StatusCode {
                        fn from(val: i32) -> Self {
                            Self(val)
                        }
                    }
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Status {
                    pub r#message: crate::bumpalo::UnsafeString,
                    pub r#code: Status_::StatusCode,
                }
                impl ::core::default::Default for Status {
                    fn default() -> Self {
                        Self {
                            r#message: ::core::default::Default::default(),
                            r#code: ::core::default::Default::default(),
                        }
                    }
                }
                impl Status {}
                impl ::micropb::MessageDecode for Status {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                2u32 => {
                                    let mut_ref = &mut self.r#message;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                3u32 => {
                                    let mut_ref = &mut self.r#code;
                                    {
                                        let val = decoder
                                            .decode_int32()
                                            .map(|n| Status_::StatusCode(n as _))?;
                                        let val_ref = &val;
                                        if val_ref.0 != 0 {
                                            *mut_ref = val as _;
                                        }
                                    };
                                }
                                _ => {
                                    decoder.skip_wire_value(tag.wire_type())?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
                impl ::micropb::MessageEncode for Status {
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        {
                            let val_ref = &self.r#message;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(18u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#code;
                            if val_ref.0 != 0 {
                                encoder.encode_varint32(24u32)?;
                                encoder.encode_int32(val_ref.0 as _)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                        let mut size = 0;
                        {
                            let val_ref = &self.r#message;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#code;
                            if val_ref.0 != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
                            }
                        }
                        size
                    }
                }
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct SpanFlags(pub i32);
                impl SpanFlags {
                    pub const DoNotUse: Self = Self(0);
                    pub const TraceFlagsMask: Self = Self(255);
                    pub const ContextHasIsRemoteMask: Self = Self(256);
                    pub const ContextIsRemoteMask: Self = Self(512);
                }
                impl core::default::Default for SpanFlags {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl core::convert::From<i32> for SpanFlags {
                    fn from(val: i32) -> Self {
                        Self(val)
                    }
                }
            }
        }
    }
}
