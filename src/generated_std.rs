pub mod opentelemetry_ {
    pub mod proto_ {
        pub mod common_ {
            pub mod v1_ {
                /// Represents any type of attribute value. AnyValue may contain a
                /// primitive value such as a string or integer or it may contain an arbitrary nested
                /// object containing arrays, key-value lists and primitives.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct AnyValue {
                    /// The value is one of the listed fields. It is valid for all values to be unspecified
                    /// in which case this AnyValue is considered to be "empty".
                    pub r#value: ::core::option::Option<AnyValue_::Value>,
                }
                impl AnyValue {}
                impl ::micropb::MessageDecode for AnyValue {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                                8u32 => {
                                    let mut_ref = loop {
                                        if let ::core::option::Option::Some(variant) = &mut self
                                            .r#value
                                        {
                                            if let AnyValue_::Value::StringValueStrindex(variant) = &mut *variant {
                                                break &mut *variant;
                                            }
                                        }
                                        self.r#value = ::core::option::Option::Some(
                                            AnyValue_::Value::StringValueStrindex(
                                                ::core::default::Default::default(),
                                            ),
                                        );
                                    };
                                    let val = decoder.decode_int32()?;
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
                impl ::micropb::MessageEncode for AnyValue {
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match 'oneof: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.common.v1.AnyValue.string_value) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(1usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(10usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< ArrayValue as
                                ::micropb::MessageEncode > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< KeyValueList as
                                ::micropb::MessageEncode > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.common.v1.AnyValue.bytes_value) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(10usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        } {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
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
                                AnyValue_::Value::StringValueStrindex(val_ref) => {
                                    let val_ref = &*val_ref;
                                    encoder.encode_varint32(64u32)?;
                                    encoder.encode_int32(*val_ref as _)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
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
                                AnyValue_::Value::StringValueStrindex(val_ref) => {
                                    let val_ref = &*val_ref;
                                    size
                                        += 1usize + ::micropb::size::sizeof_int32(*val_ref as _);
                                }
                            }
                        }
                        size
                    }
                }
                /// Inner types for `AnyValue`
                pub mod AnyValue_ {
                    /// The value is one of the listed fields. It is valid for all values to be unspecified
                    /// in which case this AnyValue is considered to be "empty".
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Value {
                        StringValue(::std::string::String),
                        BoolValue(bool),
                        IntValue(i64),
                        DoubleValue(f64),
                        ArrayValue(super::ArrayValue),
                        KvlistValue(super::KeyValueList),
                        BytesValue(::std::vec::Vec<u8>),
                        /// Reference to the string value in ProfilesDictionary.string_table.
                        ///
                        /// Note: This is currently used exclusively in the Profiling signal.
                        /// Implementers of OTLP receivers for signals other than Profiling should
                        /// treat the presence of this value as a non-fatal issue.
                        /// Log an error or warning indicating an unexpected field intended for the
                        /// Profiling signal and process the data as if this value were absent or
                        /// empty, ignoring its semantic content for the non-Profiling signal.
                        ///
                        /// Status: [Alpha]
                        StringValueStrindex(i32),
                    }
                }
                /// ArrayValue is a list of AnyValue messages. We need ArrayValue as a message
                /// since oneof in AnyValue does not allow repeated fields.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct ArrayValue {
                    /// Array of values. The array may be empty (contain 0 elements).
                    pub r#values: ::std::vec::Vec<AnyValue>,
                }
                impl ArrayValue {}
                impl ::micropb::MessageDecode for ArrayValue {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.common.v1.ArrayValue.values) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#values.iter().enumerate() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#values.iter().enumerate() {
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
                /// KeyValueList is a list of KeyValue messages. We need KeyValueList as a message
                /// since `oneof` in AnyValue does not allow repeated fields. Everywhere else where we need
                /// a list of KeyValue messages (e.g. in Span) we use `repeated KeyValue` directly to
                /// avoid unnecessary extra wrapping (which slows down the protocol). The 2 approaches
                /// are semantically equivalent.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct KeyValueList {
                    /// A collection of key/value pairs of key-value pairs. The list may be empty (may
                    /// contain 0 elements).
                    ///
                    /// The keys MUST be unique (it is not allowed to have more than one
                    /// value with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#values: ::std::vec::Vec<KeyValue>,
                }
                impl KeyValueList {}
                impl ::micropb::MessageDecode for KeyValueList {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.common.v1.KeyValueList.values) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#values.iter().enumerate() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#values.iter().enumerate() {
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
                /// Represents a key-value pair that is used to store Span attributes, Link
                /// attributes, etc.
                #[derive(Debug, Default, Clone)]
                pub struct KeyValue {
                    /// The key name of the pair.
                    /// key_strindex MUST NOT be set if key is used.
                    pub r#key: ::std::string::String,
                    /// The value of the pair.
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#value: AnyValue,
                    /// Reference to the string key in ProfilesDictionary.string_table.
                    /// key MUST NOT be set if key_strindex is used.
                    ///
                    /// Note: This is currently used exclusively in the Profiling signal.
                    /// Implementers of OTLP receivers for signals other than Profiling should
                    /// treat the presence of this key as a non-fatal issue.
                    /// Log an error or warning indicating an unexpected field intended for the
                    /// Profiling signal and process the data as if this value were absent or
                    /// empty, ignoring its semantic content for the non-Profiling signal.
                    ///
                    /// Status: [Alpha]
                    pub r#key_strindex: i32,
                    /// Tracks presence of optional and message fields
                    pub _has: KeyValue_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for KeyValue {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#key == other.r#key);
                        ret &= (self.r#value() == other.r#value());
                        ret &= (self.r#key_strindex == other.r#key_strindex);
                        ret
                    }
                }
                impl KeyValue {
                    /// Return a reference to `key`
                    #[inline]
                    pub fn r#key(&self) -> &::std::string::String {
                        &self.r#key
                    }
                    /// Return a mutable reference to `key`
                    #[inline]
                    pub fn mut_key(&mut self) -> &mut ::std::string::String {
                        &mut self.r#key
                    }
                    /// Set the value of `key`
                    #[inline]
                    pub fn set_key(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#key = value.into();
                        self
                    }
                    /// Builder method that sets the value of `key`. Useful for initializing the message.
                    #[inline]
                    pub fn init_key(mut self, value: ::std::string::String) -> Self {
                        self.r#key = value.into();
                        self
                    }
                    /// Return a reference to `value` as an `Option`
                    #[inline]
                    pub fn r#value(&self) -> ::core::option::Option<&AnyValue> {
                        self._has.r#value().then_some(&self.r#value)
                    }
                    /// Set the value and presence of `value`
                    #[inline]
                    pub fn set_value(&mut self, value: AnyValue) -> &mut Self {
                        self._has.set_value();
                        self.r#value = value.into();
                        self
                    }
                    /// Return a mutable reference to `value` as an `Option`
                    #[inline]
                    pub fn mut_value(
                        &mut self,
                    ) -> ::core::option::Option<&mut AnyValue> {
                        self._has.r#value().then_some(&mut self.r#value)
                    }
                    /// Clear the presence of `value`
                    #[inline]
                    pub fn clear_value(&mut self) -> &mut Self {
                        self._has.clear_value();
                        self
                    }
                    /// Take the value of `value` and clear its presence
                    #[inline]
                    pub fn take_value(&mut self) -> ::core::option::Option<AnyValue> {
                        let val = self
                            ._has
                            .r#value()
                            .then(|| ::core::mem::take(&mut self.r#value));
                        self._has.clear_value();
                        val
                    }
                    /// Builder method that sets the value of `value`. Useful for initializing the message.
                    #[inline]
                    pub fn init_value(mut self, value: AnyValue) -> Self {
                        self.set_value(value);
                        self
                    }
                    /// Return a reference to `key_strindex`
                    #[inline]
                    pub fn r#key_strindex(&self) -> &i32 {
                        &self.r#key_strindex
                    }
                    /// Return a mutable reference to `key_strindex`
                    #[inline]
                    pub fn mut_key_strindex(&mut self) -> &mut i32 {
                        &mut self.r#key_strindex
                    }
                    /// Set the value of `key_strindex`
                    #[inline]
                    pub fn set_key_strindex(&mut self, value: i32) -> &mut Self {
                        self.r#key_strindex = value.into();
                        self
                    }
                    /// Builder method that sets the value of `key_strindex`. Useful for initializing the message.
                    #[inline]
                    pub fn init_key_strindex(mut self, value: i32) -> Self {
                        self.r#key_strindex = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for KeyValue {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                                3u32 => {
                                    let mut_ref = &mut self.r#key_strindex;
                                    {
                                        let val = decoder.decode_int32()?;
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
                impl ::micropb::MessageEncode for KeyValue {
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.common.v1.KeyValue.key) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::micropb::const_map!(< AnyValue as ::micropb::MessageEncode
                            > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(10usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            let val_ref = &self.r#key;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(10u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#value()
                            {
                                encoder.encode_varint32(18u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            let val_ref = &self.r#key_strindex;
                            if *val_ref != 0 {
                                encoder.encode_varint32(24u32)?;
                                encoder.encode_int32(*val_ref as _)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
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
                            if let ::core::option::Option::Some(val_ref) = self.r#value()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            let val_ref = &self.r#key_strindex;
                            if *val_ref != 0 {
                                size
                                    += 1usize + ::micropb::size::sizeof_int32(*val_ref as _);
                            }
                        }
                        size
                    }
                }
                /// Inner types for `KeyValue`
                pub mod KeyValue_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `value`
                        #[inline]
                        pub const fn r#value(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `value`
                        #[inline]
                        pub const fn set_value(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `value`
                        #[inline]
                        pub const fn clear_value(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `value`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_value(mut self) -> Self {
                            self.set_value();
                            self
                        }
                    }
                }
                /// InstrumentationScope is a message representing the instrumentation scope information
                /// such as the fully qualified name and version.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct InstrumentationScope {
                    /// A name denoting the Instrumentation scope.
                    /// An empty instrumentation scope name means the name is unknown.
                    pub r#name: ::std::string::String,
                    /// Defines the version of the instrumentation scope.
                    /// An empty instrumentation scope version means the version is unknown.
                    pub r#version: ::std::string::String,
                    /// Additional attributes that describe the scope. [Optional].
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<KeyValue>,
                    /// The number of attributes that were discarded. Attributes
                    /// can be discarded because their keys are too long or because there are too many
                    /// attributes. If this value is 0, then no attributes were dropped.
                    pub r#dropped_attributes_count: u32,
                }
                impl InstrumentationScope {
                    /// Return a reference to `name`
                    #[inline]
                    pub fn r#name(&self) -> &::std::string::String {
                        &self.r#name
                    }
                    /// Return a mutable reference to `name`
                    #[inline]
                    pub fn mut_name(&mut self) -> &mut ::std::string::String {
                        &mut self.r#name
                    }
                    /// Set the value of `name`
                    #[inline]
                    pub fn set_name(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#name = value.into();
                        self
                    }
                    /// Builder method that sets the value of `name`. Useful for initializing the message.
                    #[inline]
                    pub fn init_name(mut self, value: ::std::string::String) -> Self {
                        self.r#name = value.into();
                        self
                    }
                    /// Return a reference to `version`
                    #[inline]
                    pub fn r#version(&self) -> &::std::string::String {
                        &self.r#version
                    }
                    /// Return a mutable reference to `version`
                    #[inline]
                    pub fn mut_version(&mut self) -> &mut ::std::string::String {
                        &mut self.r#version
                    }
                    /// Set the value of `version`
                    #[inline]
                    pub fn set_version(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#version = value.into();
                        self
                    }
                    /// Builder method that sets the value of `version`. Useful for initializing the message.
                    #[inline]
                    pub fn init_version(mut self, value: ::std::string::String) -> Self {
                        self.r#version = value.into();
                        self
                    }
                    /// Return a reference to `dropped_attributes_count`
                    #[inline]
                    pub fn r#dropped_attributes_count(&self) -> &u32 {
                        &self.r#dropped_attributes_count
                    }
                    /// Return a mutable reference to `dropped_attributes_count`
                    #[inline]
                    pub fn mut_dropped_attributes_count(&mut self) -> &mut u32 {
                        &mut self.r#dropped_attributes_count
                    }
                    /// Set the value of `dropped_attributes_count`
                    #[inline]
                    pub fn set_dropped_attributes_count(
                        &mut self,
                        value: u32,
                    ) -> &mut Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `dropped_attributes_count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_dropped_attributes_count(mut self, value: u32) -> Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for InstrumentationScope {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.common.v1.InstrumentationScope.name) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.common.v1.InstrumentationScope.version) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.common.v1.InstrumentationScope.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
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
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
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
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                /// A reference to an Entity.
                /// Entity represents an object of interest associated with produced telemetry: e.g spans, metrics, profiles, or logs.
                ///
                /// Status: [Development]
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct EntityRef {
                    /// The Schema URL, if known. This is the identifier of the Schema that the entity data
                    /// is recorded in. To learn more about Schema URL see
                    /// https://opentelemetry.io/docs/specs/otel/schemas/#schema-url
                    ///
                    /// This schema_url applies to the data in this message and to the Resource attributes
                    /// referenced by id_keys and description_keys.
                    /// TODO: discuss if we are happy with this somewhat complicated definition of what
                    /// the schema_url applies to.
                    ///
                    /// This field obsoletes the schema_url field in ResourceMetrics/ResourceSpans/ResourceLogs.
                    pub r#schema_url: ::std::string::String,
                    /// Defines the type of the entity. MUST not change during the lifetime of the entity.
                    /// For example: "service" or "host". This field is required and MUST not be empty
                    /// for valid entities.
                    pub r#type: ::std::string::String,
                    /// Attribute Keys that identify the entity.
                    /// MUST not change during the lifetime of the entity. The Id must contain at least one attribute.
                    /// These keys MUST exist in the containing {message}.attributes.
                    pub r#id_keys: ::std::vec::Vec<::std::string::String>,
                    /// Descriptive (non-identifying) attribute keys of the entity.
                    /// MAY change over the lifetime of the entity. MAY be empty.
                    /// These attribute keys are not part of entity's identity.
                    /// These keys MUST exist in the containing {message}.attributes.
                    pub r#description_keys: ::std::vec::Vec<::std::string::String>,
                }
                impl EntityRef {
                    /// Return a reference to `schema_url`
                    #[inline]
                    pub fn r#schema_url(&self) -> &::std::string::String {
                        &self.r#schema_url
                    }
                    /// Return a mutable reference to `schema_url`
                    #[inline]
                    pub fn mut_schema_url(&mut self) -> &mut ::std::string::String {
                        &mut self.r#schema_url
                    }
                    /// Set the value of `schema_url`
                    #[inline]
                    pub fn set_schema_url(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Builder method that sets the value of `schema_url`. Useful for initializing the message.
                    #[inline]
                    pub fn init_schema_url(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Return a reference to `type`
                    #[inline]
                    pub fn r#type(&self) -> &::std::string::String {
                        &self.r#type
                    }
                    /// Return a mutable reference to `type`
                    #[inline]
                    pub fn mut_type(&mut self) -> &mut ::std::string::String {
                        &mut self.r#type
                    }
                    /// Set the value of `type`
                    #[inline]
                    pub fn set_type(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#type = value.into();
                        self
                    }
                    /// Builder method that sets the value of `type`. Useful for initializing the message.
                    #[inline]
                    pub fn init_type(mut self, value: ::std::string::String) -> Self {
                        self.r#type = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for EntityRef {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
                        let before = decoder.bytes_read();
                        while decoder.bytes_read() - before < len {
                            let tag = decoder.decode_tag()?;
                            match tag.field_num() {
                                0 => return Err(::micropb::DecodeError::ZeroField),
                                1u32 => {
                                    let mut_ref = &mut self.r#schema_url;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                2u32 => {
                                    let mut_ref = &mut self.r#type;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                                    };
                                }
                                3u32 => {
                                    let mut val: ::std::string::String = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#id_keys.pb_push(val),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                                4u32 => {
                                    let mut val: ::std::string::String = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        decoder
                                            .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#description_keys.pb_push(val),
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
                impl ::micropb::MessageEncode for EntityRef {
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.common.v1.EntityRef.schema_url) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.common.v1.EntityRef.type) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.common.v1.EntityRef.id_keys) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.common.v1.EntityRef.description_keys) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(10u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            let val_ref = &self.r#type;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(18u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#id_keys.iter().enumerate() {
                                encoder.encode_varint32(26u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        {
                            for (i, val_ref) in self
                                .r#description_keys
                                .iter()
                                .enumerate()
                            {
                                encoder.encode_varint32(34u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            let val_ref = &self.r#schema_url;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            let val_ref = &self.r#type;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            for (i, val_ref) in self.r#id_keys.iter().enumerate() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        {
                            for (i, val_ref) in self
                                .r#description_keys
                                .iter()
                                .enumerate()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
            }
        }
        pub mod resource_ {
            pub mod v1_ {
                /// Resource information.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Resource {
                    /// Set of attributes that describe the resource.
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// The number of dropped attributes. If the value is 0, then
                    /// no attributes were dropped.
                    pub r#dropped_attributes_count: u32,
                    /// Set of entities that participate in this Resource.
                    ///
                    /// Note: keys in the references MUST exist in attributes of this message.
                    ///
                    /// Status: [Development]
                    pub r#entity_refs: ::std::vec::Vec<
                        super::super::common_::v1_::EntityRef,
                    >,
                }
                impl Resource {
                    /// Return a reference to `dropped_attributes_count`
                    #[inline]
                    pub fn r#dropped_attributes_count(&self) -> &u32 {
                        &self.r#dropped_attributes_count
                    }
                    /// Return a mutable reference to `dropped_attributes_count`
                    #[inline]
                    pub fn mut_dropped_attributes_count(&mut self) -> &mut u32 {
                        &mut self.r#dropped_attributes_count
                    }
                    /// Set the value of `dropped_attributes_count`
                    #[inline]
                    pub fn set_dropped_attributes_count(
                        &mut self,
                        value: u32,
                    ) -> &mut Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `dropped_attributes_count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_dropped_attributes_count(mut self, value: u32) -> Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for Resource {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                                3u32 => {
                                    let mut val: super::super::common_::v1_::EntityRef = ::core::default::Default::default();
                                    let mut_ref = &mut val;
                                    {
                                        mut_ref.decode_len_delimited(decoder)?;
                                    };
                                    if let (Err(_), false) = (
                                        self.r#entity_refs.pb_push(val),
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
                impl ::micropb::MessageEncode for Resource {
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.resource.v1.Resource.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.resource.v1.Resource.entity_refs) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                        {
                            for (i, val_ref) in self.r#entity_refs.iter().enumerate() {
                                encoder.encode_varint32(26u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            for (i, val_ref) in self.r#entity_refs.iter().enumerate() {
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
            }
        }
        pub mod logs_ {
            pub mod v1_ {
                /// LogsData represents the logs data that can be stored in a persistent storage,
                /// OR can be embedded by other protocols that transfer OTLP logs data but do not
                /// implement the OTLP protocol.
                ///
                /// The main difference between this message and collector protocol is that
                /// in this message there will not be any "control" or "metadata" specific to
                /// OTLP protocol.
                ///
                /// When new fields are added into this message, the OTLP request MUST be updated
                /// as well.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct LogsData {
                    /// An array of ResourceLogs.
                    /// For data coming from a single resource this array will typically contain
                    /// one element. Intermediary nodes that receive data from multiple origins
                    /// typically batch the data before forwarding further and in that case this
                    /// array will contain multiple elements.
                    pub r#resource_logs: ::std::vec::Vec<ResourceLogs>,
                }
                impl LogsData {}
                impl ::micropb::MessageDecode for LogsData {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.logs.v1.LogsData.resource_logs) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#resource_logs.iter().enumerate() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#resource_logs.iter().enumerate() {
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
                /// A collection of ScopeLogs from a Resource.
                #[derive(Debug, Default, Clone)]
                pub struct ResourceLogs {
                    /// The resource for the logs in this message.
                    /// If this field is not set then resource info is unknown.
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#resource: super::super::resource_::v1_::Resource,
                    /// A list of ScopeLogs that originate from a resource.
                    pub r#scope_logs: ::std::vec::Vec<ScopeLogs>,
                    /// The Schema URL, if known. This is the identifier of the Schema that the resource data
                    /// is recorded in. Notably, the last part of the URL path is the version number of the
                    /// schema: http[s]://server[:port]/path/<version>. To learn more about Schema URL see
                    /// https://opentelemetry.io/docs/specs/otel/schemas/#schema-url
                    /// This schema_url applies to the data in the "resource" field. It does not apply
                    /// to the data in the "scope_logs" field which have their own schema_url field.
                    pub r#schema_url: ::std::string::String,
                    /// Tracks presence of optional and message fields
                    pub _has: ResourceLogs_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for ResourceLogs {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#resource() == other.r#resource());
                        ret &= (self.r#scope_logs == other.r#scope_logs);
                        ret &= (self.r#schema_url == other.r#schema_url);
                        ret
                    }
                }
                impl ResourceLogs {
                    /// Return a reference to `resource` as an `Option`
                    #[inline]
                    pub fn r#resource(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&self.r#resource)
                    }
                    /// Set the value and presence of `resource`
                    #[inline]
                    pub fn set_resource(
                        &mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) -> &mut Self {
                        self._has.set_resource();
                        self.r#resource = value.into();
                        self
                    }
                    /// Return a mutable reference to `resource` as an `Option`
                    #[inline]
                    pub fn mut_resource(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&mut self.r#resource)
                    }
                    /// Clear the presence of `resource`
                    #[inline]
                    pub fn clear_resource(&mut self) -> &mut Self {
                        self._has.clear_resource();
                        self
                    }
                    /// Take the value of `resource` and clear its presence
                    #[inline]
                    pub fn take_resource(
                        &mut self,
                    ) -> ::core::option::Option<super::super::resource_::v1_::Resource> {
                        let val = self
                            ._has
                            .r#resource()
                            .then(|| ::core::mem::take(&mut self.r#resource));
                        self._has.clear_resource();
                        val
                    }
                    /// Builder method that sets the value of `resource`. Useful for initializing the message.
                    #[inline]
                    pub fn init_resource(
                        mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) -> Self {
                        self.set_resource(value);
                        self
                    }
                    /// Return a reference to `schema_url`
                    #[inline]
                    pub fn r#schema_url(&self) -> &::std::string::String {
                        &self.r#schema_url
                    }
                    /// Return a mutable reference to `schema_url`
                    #[inline]
                    pub fn mut_schema_url(&mut self) -> &mut ::std::string::String {
                        &mut self.r#schema_url
                    }
                    /// Set the value of `schema_url`
                    #[inline]
                    pub fn set_schema_url(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Builder method that sets the value of `schema_url`. Useful for initializing the message.
                    #[inline]
                    pub fn init_schema_url(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#schema_url = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ResourceLogs {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            super::super::resource_::v1_::Resource as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.logs.v1.ResourceLogs.scope_logs) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.logs.v1.ResourceLogs.schema_url) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#resource()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#scope_logs.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#resource()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for (i, val_ref) in self.r#scope_logs.iter().enumerate() {
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
                /// Inner types for `ResourceLogs`
                pub mod ResourceLogs_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `resource`
                        #[inline]
                        pub const fn r#resource(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `resource`
                        #[inline]
                        pub const fn set_resource(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `resource`
                        #[inline]
                        pub const fn clear_resource(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `resource`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_resource(mut self) -> Self {
                            self.set_resource();
                            self
                        }
                    }
                }
                /// A collection of Logs produced by a Scope.
                #[derive(Debug, Default, Clone)]
                pub struct ScopeLogs {
                    /// The instrumentation scope information for the logs in this message.
                    /// Semantically when InstrumentationScope isn't set, it is equivalent with
                    /// an empty instrumentation scope name (unknown).
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#scope: super::super::common_::v1_::InstrumentationScope,
                    /// A list of log records.
                    pub r#log_records: ::std::vec::Vec<LogRecord>,
                    /// The Schema URL, if known. This is the identifier of the Schema that the log data
                    /// is recorded in. Notably, the last part of the URL path is the version number of the
                    /// schema: http[s]://server[:port]/path/<version>. To learn more about Schema URL see
                    /// https://opentelemetry.io/docs/specs/otel/schemas/#schema-url
                    /// This schema_url applies to the data in the "scope" field and all logs in the
                    /// "log_records" field.
                    pub r#schema_url: ::std::string::String,
                    /// Tracks presence of optional and message fields
                    pub _has: ScopeLogs_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for ScopeLogs {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#scope() == other.r#scope());
                        ret &= (self.r#log_records == other.r#log_records);
                        ret &= (self.r#schema_url == other.r#schema_url);
                        ret
                    }
                }
                impl ScopeLogs {
                    /// Return a reference to `scope` as an `Option`
                    #[inline]
                    pub fn r#scope(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&self.r#scope)
                    }
                    /// Set the value and presence of `scope`
                    #[inline]
                    pub fn set_scope(
                        &mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) -> &mut Self {
                        self._has.set_scope();
                        self.r#scope = value.into();
                        self
                    }
                    /// Return a mutable reference to `scope` as an `Option`
                    #[inline]
                    pub fn mut_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&mut self.r#scope)
                    }
                    /// Clear the presence of `scope`
                    #[inline]
                    pub fn clear_scope(&mut self) -> &mut Self {
                        self._has.clear_scope();
                        self
                    }
                    /// Take the value of `scope` and clear its presence
                    #[inline]
                    pub fn take_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        super::super::common_::v1_::InstrumentationScope,
                    > {
                        let val = self
                            ._has
                            .r#scope()
                            .then(|| ::core::mem::take(&mut self.r#scope));
                        self._has.clear_scope();
                        val
                    }
                    /// Builder method that sets the value of `scope`. Useful for initializing the message.
                    #[inline]
                    pub fn init_scope(
                        mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) -> Self {
                        self.set_scope(value);
                        self
                    }
                    /// Return a reference to `schema_url`
                    #[inline]
                    pub fn r#schema_url(&self) -> &::std::string::String {
                        &self.r#schema_url
                    }
                    /// Return a mutable reference to `schema_url`
                    #[inline]
                    pub fn mut_schema_url(&mut self) -> &mut ::std::string::String {
                        &mut self.r#schema_url
                    }
                    /// Set the value of `schema_url`
                    #[inline]
                    pub fn set_schema_url(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Builder method that sets the value of `schema_url`. Useful for initializing the message.
                    #[inline]
                    pub fn init_schema_url(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#schema_url = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ScopeLogs {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            super::super::common_::v1_::InstrumentationScope as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.logs.v1.ScopeLogs.log_records) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.logs.v1.ScopeLogs.schema_url) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#scope()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#log_records.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#scope()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for (i, val_ref) in self.r#log_records.iter().enumerate() {
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
                /// Inner types for `ScopeLogs`
                pub mod ScopeLogs_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `scope`
                        #[inline]
                        pub const fn r#scope(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `scope`
                        #[inline]
                        pub const fn set_scope(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `scope`
                        #[inline]
                        pub const fn clear_scope(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `scope`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_scope(mut self) -> Self {
                            self.set_scope();
                            self
                        }
                    }
                }
                /// A log record according to OpenTelemetry Log Data Model:
                /// https://github.com/open-telemetry/oteps/blob/main/text/logs/0097-log-data-model.md
                #[derive(Debug, Default, Clone)]
                pub struct LogRecord {
                    /// time_unix_nano is the time when the event occurred.
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
                    /// Value of 0 indicates unknown or missing timestamp.
                    pub r#time_unix_nano: u64,
                    /// Time when the event was observed by the collection system.
                    /// For events that originate in OpenTelemetry (e.g. using OpenTelemetry Logging SDK)
                    /// this timestamp is typically set at the generation time and is equal to Timestamp.
                    /// For events originating externally and collected by OpenTelemetry (e.g. using
                    /// Collector) this is the time when OpenTelemetry's code observed the event measured
                    /// by the clock of the OpenTelemetry code. This field MUST be set once the event is
                    /// observed by OpenTelemetry.
                    ///
                    /// For converting OpenTelemetry log data to formats that support only one timestamp or
                    /// when receiving OpenTelemetry log data by recipients that support only one timestamp
                    /// internally the following logic is recommended:
                    ///   - Use time_unix_nano if it is present, otherwise use observed_time_unix_nano.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
                    /// Value of 0 indicates unknown or missing timestamp.
                    pub r#observed_time_unix_nano: u64,
                    /// Numerical value of the severity, normalized to values described in Log Data Model.
                    /// [Optional].
                    pub r#severity_number: SeverityNumber,
                    /// The severity text (also known as log level). The original string representation as
                    /// it is known at the source. [Optional].
                    pub r#severity_text: ::std::string::String,
                    /// A value containing the body of the log record. Can be for example a human-readable
                    /// string message (including multi-line) describing the event in a free form or it can
                    /// be a structured data composed of arrays and maps of other values. [Optional].
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#body: super::super::common_::v1_::AnyValue,
                    /// Additional attributes that describe the specific event occurrence. [Optional].
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    pub r#dropped_attributes_count: u32,
                    /// Flags, a bit field. 8 least significant bits are the trace flags as
                    /// defined in W3C Trace Context specification. 24 most significant bits are reserved
                    /// and must be set to 0. Readers must not assume that 24 most significant bits
                    /// will be zero and must correctly mask the bits when reading 8-bit trace flag (use
                    /// flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK). [Optional].
                    pub r#flags: u32,
                    /// A unique identifier for a trace. All logs from the same trace share
                    /// the same `trace_id`. The ID is a 16-byte array. An ID with all zeroes OR
                    /// of length other than 16 bytes is considered invalid (empty string in OTLP/JSON
                    /// is zero-length and thus is also invalid).
                    ///
                    /// This field is optional.
                    ///
                    /// The receivers SHOULD assume that the log record is not associated with a
                    /// trace if any of the following is true:
                    ///   - the field is not present,
                    ///   - the field contains an invalid value.
                    pub r#trace_id: ::std::vec::Vec<u8>,
                    /// A unique identifier for a span within a trace, assigned when the span
                    /// is created. The ID is an 8-byte array. An ID with all zeroes OR of length
                    /// other than 8 bytes is considered invalid (empty string in OTLP/JSON
                    /// is zero-length and thus is also invalid).
                    ///
                    /// This field is optional. If the sender specifies a valid span_id then it SHOULD also
                    /// specify a valid trace_id.
                    ///
                    /// The receivers SHOULD assume that the log record is not associated with a
                    /// span if any of the following is true:
                    ///   - the field is not present,
                    ///   - the field contains an invalid value.
                    pub r#span_id: ::std::vec::Vec<u8>,
                    /// A unique identifier of event category/type.
                    /// All events with the same event_name are expected to conform to the same
                    /// schema for both their attributes and their body.
                    ///
                    /// Recommended to be fully qualified and short (no longer than 256 characters).
                    ///
                    /// Presence of event_name on the log record identifies this record
                    /// as an event.
                    ///
                    /// [Optional].
                    pub r#event_name: ::std::string::String,
                    /// Tracks presence of optional and message fields
                    pub _has: LogRecord_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for LogRecord {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#time_unix_nano == other.r#time_unix_nano);
                        ret
                            &= (self.r#observed_time_unix_nano
                                == other.r#observed_time_unix_nano);
                        ret &= (self.r#severity_number == other.r#severity_number);
                        ret &= (self.r#severity_text == other.r#severity_text);
                        ret &= (self.r#body() == other.r#body());
                        ret &= (self.r#attributes == other.r#attributes);
                        ret
                            &= (self.r#dropped_attributes_count
                                == other.r#dropped_attributes_count);
                        ret &= (self.r#flags == other.r#flags);
                        ret &= (self.r#trace_id == other.r#trace_id);
                        ret &= (self.r#span_id == other.r#span_id);
                        ret &= (self.r#event_name == other.r#event_name);
                        ret
                    }
                }
                impl LogRecord {
                    /// Return a reference to `time_unix_nano`
                    #[inline]
                    pub fn r#time_unix_nano(&self) -> &u64 {
                        &self.r#time_unix_nano
                    }
                    /// Return a mutable reference to `time_unix_nano`
                    #[inline]
                    pub fn mut_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#time_unix_nano
                    }
                    /// Set the value of `time_unix_nano`
                    #[inline]
                    pub fn set_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `observed_time_unix_nano`
                    #[inline]
                    pub fn r#observed_time_unix_nano(&self) -> &u64 {
                        &self.r#observed_time_unix_nano
                    }
                    /// Return a mutable reference to `observed_time_unix_nano`
                    #[inline]
                    pub fn mut_observed_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#observed_time_unix_nano
                    }
                    /// Set the value of `observed_time_unix_nano`
                    #[inline]
                    pub fn set_observed_time_unix_nano(
                        &mut self,
                        value: u64,
                    ) -> &mut Self {
                        self.r#observed_time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `observed_time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_observed_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#observed_time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `severity_number`
                    #[inline]
                    pub fn r#severity_number(&self) -> &SeverityNumber {
                        &self.r#severity_number
                    }
                    /// Return a mutable reference to `severity_number`
                    #[inline]
                    pub fn mut_severity_number(&mut self) -> &mut SeverityNumber {
                        &mut self.r#severity_number
                    }
                    /// Set the value of `severity_number`
                    #[inline]
                    pub fn set_severity_number(
                        &mut self,
                        value: SeverityNumber,
                    ) -> &mut Self {
                        self.r#severity_number = value.into();
                        self
                    }
                    /// Builder method that sets the value of `severity_number`. Useful for initializing the message.
                    #[inline]
                    pub fn init_severity_number(
                        mut self,
                        value: SeverityNumber,
                    ) -> Self {
                        self.r#severity_number = value.into();
                        self
                    }
                    /// Return a reference to `severity_text`
                    #[inline]
                    pub fn r#severity_text(&self) -> &::std::string::String {
                        &self.r#severity_text
                    }
                    /// Return a mutable reference to `severity_text`
                    #[inline]
                    pub fn mut_severity_text(&mut self) -> &mut ::std::string::String {
                        &mut self.r#severity_text
                    }
                    /// Set the value of `severity_text`
                    #[inline]
                    pub fn set_severity_text(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#severity_text = value.into();
                        self
                    }
                    /// Builder method that sets the value of `severity_text`. Useful for initializing the message.
                    #[inline]
                    pub fn init_severity_text(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#severity_text = value.into();
                        self
                    }
                    /// Return a reference to `body` as an `Option`
                    #[inline]
                    pub fn r#body(
                        &self,
                    ) -> ::core::option::Option<&super::super::common_::v1_::AnyValue> {
                        self._has.r#body().then_some(&self.r#body)
                    }
                    /// Set the value and presence of `body`
                    #[inline]
                    pub fn set_body(
                        &mut self,
                        value: super::super::common_::v1_::AnyValue,
                    ) -> &mut Self {
                        self._has.set_body();
                        self.r#body = value.into();
                        self
                    }
                    /// Return a mutable reference to `body` as an `Option`
                    #[inline]
                    pub fn mut_body(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::AnyValue,
                    > {
                        self._has.r#body().then_some(&mut self.r#body)
                    }
                    /// Clear the presence of `body`
                    #[inline]
                    pub fn clear_body(&mut self) -> &mut Self {
                        self._has.clear_body();
                        self
                    }
                    /// Take the value of `body` and clear its presence
                    #[inline]
                    pub fn take_body(
                        &mut self,
                    ) -> ::core::option::Option<super::super::common_::v1_::AnyValue> {
                        let val = self
                            ._has
                            .r#body()
                            .then(|| ::core::mem::take(&mut self.r#body));
                        self._has.clear_body();
                        val
                    }
                    /// Builder method that sets the value of `body`. Useful for initializing the message.
                    #[inline]
                    pub fn init_body(
                        mut self,
                        value: super::super::common_::v1_::AnyValue,
                    ) -> Self {
                        self.set_body(value);
                        self
                    }
                    /// Return a reference to `dropped_attributes_count`
                    #[inline]
                    pub fn r#dropped_attributes_count(&self) -> &u32 {
                        &self.r#dropped_attributes_count
                    }
                    /// Return a mutable reference to `dropped_attributes_count`
                    #[inline]
                    pub fn mut_dropped_attributes_count(&mut self) -> &mut u32 {
                        &mut self.r#dropped_attributes_count
                    }
                    /// Set the value of `dropped_attributes_count`
                    #[inline]
                    pub fn set_dropped_attributes_count(
                        &mut self,
                        value: u32,
                    ) -> &mut Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `dropped_attributes_count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_dropped_attributes_count(mut self, value: u32) -> Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                    /// Return a reference to `flags`
                    #[inline]
                    pub fn r#flags(&self) -> &u32 {
                        &self.r#flags
                    }
                    /// Return a mutable reference to `flags`
                    #[inline]
                    pub fn mut_flags(&mut self) -> &mut u32 {
                        &mut self.r#flags
                    }
                    /// Set the value of `flags`
                    #[inline]
                    pub fn set_flags(&mut self, value: u32) -> &mut Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Builder method that sets the value of `flags`. Useful for initializing the message.
                    #[inline]
                    pub fn init_flags(mut self, value: u32) -> Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Return a reference to `trace_id`
                    #[inline]
                    pub fn r#trace_id(&self) -> &::std::vec::Vec<u8> {
                        &self.r#trace_id
                    }
                    /// Return a mutable reference to `trace_id`
                    #[inline]
                    pub fn mut_trace_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                        &mut self.r#trace_id
                    }
                    /// Set the value of `trace_id`
                    #[inline]
                    pub fn set_trace_id(
                        &mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> &mut Self {
                        self.r#trace_id = value.into();
                        self
                    }
                    /// Builder method that sets the value of `trace_id`. Useful for initializing the message.
                    #[inline]
                    pub fn init_trace_id(mut self, value: ::std::vec::Vec<u8>) -> Self {
                        self.r#trace_id = value.into();
                        self
                    }
                    /// Return a reference to `span_id`
                    #[inline]
                    pub fn r#span_id(&self) -> &::std::vec::Vec<u8> {
                        &self.r#span_id
                    }
                    /// Return a mutable reference to `span_id`
                    #[inline]
                    pub fn mut_span_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                        &mut self.r#span_id
                    }
                    /// Set the value of `span_id`
                    #[inline]
                    pub fn set_span_id(
                        &mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> &mut Self {
                        self.r#span_id = value.into();
                        self
                    }
                    /// Builder method that sets the value of `span_id`. Useful for initializing the message.
                    #[inline]
                    pub fn init_span_id(mut self, value: ::std::vec::Vec<u8>) -> Self {
                        self.r#span_id = value.into();
                        self
                    }
                    /// Return a reference to `event_name`
                    #[inline]
                    pub fn r#event_name(&self) -> &::std::string::String {
                        &self.r#event_name
                    }
                    /// Return a mutable reference to `event_name`
                    #[inline]
                    pub fn mut_event_name(&mut self) -> &mut ::std::string::String {
                        &mut self.r#event_name
                    }
                    /// Set the value of `event_name`
                    #[inline]
                    pub fn set_event_name(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#event_name = value.into();
                        self
                    }
                    /// Builder method that sets the value of `event_name`. Useful for initializing the message.
                    #[inline]
                    pub fn init_event_name(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#event_name = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for LogRecord {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                                12u32 => {
                                    let mut_ref = &mut self.r#event_name;
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
                impl ::micropb::MessageEncode for LogRecord {
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(SeverityNumber::_MAX_SIZE), | size
                            | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.logs.v1.LogRecord.severity_text) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::micropb::const_map!(< super::super::common_::v1_::AnyValue
                            as ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.logs.v1.LogRecord.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(4usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.logs.v1.LogRecord.trace_id) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.logs.v1.LogRecord.span_id) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.logs.v1.LogRecord.event_name) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
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
                            if let ::core::option::Option::Some(val_ref) = self.r#body()
                            {
                                encoder.encode_varint32(42u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                        {
                            let val_ref = &self.r#event_name;
                            if !val_ref.is_empty() {
                                encoder.encode_varint32(98u32)?;
                                encoder.encode_string(val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
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
                            if let ::core::option::Option::Some(val_ref) = self.r#body()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                        {
                            let val_ref = &self.r#event_name;
                            if !val_ref.is_empty() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(val_ref.len());
                            }
                        }
                        size
                    }
                }
                /// Inner types for `LogRecord`
                pub mod LogRecord_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `body`
                        #[inline]
                        pub const fn r#body(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `body`
                        #[inline]
                        pub const fn set_body(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `body`
                        #[inline]
                        pub const fn clear_body(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `body`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_body(mut self) -> Self {
                            self.set_body();
                            self
                        }
                    }
                }
                /// Possible values for LogRecord.SeverityNumber.
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct SeverityNumber(pub i32);
                impl SeverityNumber {
                    /// Maximum encoded size of the enum
                    pub const _MAX_SIZE: usize = 10usize;
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
                /// LogRecordFlags represents constants used to interpret the
                /// LogRecord.flags field, which is protobuf 'fixed32' type and is to
                /// be used as bit-fields. Each non-zero value defined in this enum is
                /// a bit-mask.  To extract the bit-field, for example, use an
                /// expression like:
                ///
                ///   (logRecord.flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK)
                ///
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct LogRecordFlags(pub i32);
                impl LogRecordFlags {
                    /// Maximum encoded size of the enum
                    pub const _MAX_SIZE: usize = 10usize;
                    /// The zero value for the enum. Should not be used for comparisons.
                    /// Instead use bitwise "and" with the appropriate mask as shown above.
                    pub const DoNotUse: Self = Self(0);
                    /// Bits 0-7 are used for trace flags.
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
        pub mod collector_ {
            pub mod logs_ {
                pub mod v1_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct ExportLogsServiceRequest {
                        /// An array of ResourceLogs.
                        /// For data coming from a single resource this array will typically contain one
                        /// element. Intermediary nodes (such as OpenTelemetry Collector) that receive
                        /// data from multiple origins typically batch the data before forwarding further and
                        /// in that case this array will contain multiple elements.
                        pub r#resource_logs: ::std::vec::Vec<
                            super::super::super::logs_::v1_::ResourceLogs,
                        >,
                    }
                    impl ExportLogsServiceRequest {}
                    impl ::micropb::MessageDecode for ExportLogsServiceRequest {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut val: super::super::super::logs_::v1_::ResourceLogs = ::core::default::Default::default();
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
                    impl ::micropb::MessageEncode for ExportLogsServiceRequest {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::core::result::Result::<
                                usize,
                                &'static str,
                            >::Err(
                                "(.opentelemetry.proto.collector.logs.v1.ExportLogsServiceRequest.resource_logs) unbounded vec",
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                for (i, val_ref) in self.r#resource_logs.iter().enumerate()
                                {
                                    encoder.encode_varint32(10u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                for (i, val_ref) in self.r#resource_logs.iter().enumerate()
                                {
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
                    #[derive(Debug, Default, Clone)]
                    pub struct ExportLogsServiceResponse {
                        /// The details of a partially successful export request.
                        ///
                        /// If the request is only partially accepted
                        /// (i.e. when the server accepts only parts of the data and rejects the rest)
                        /// the server MUST initialize the `partial_success` field and MUST
                        /// set the `rejected_<signal>` with the number of items it rejected.
                        ///
                        /// Servers MAY also make use of the `partial_success` field to convey
                        /// warnings/suggestions to senders even when the request was fully accepted.
                        /// In such cases, the `rejected_<signal>` MUST have a value of `0` and
                        /// the `error_message` MUST be non-empty.
                        ///
                        /// A `partial_success` message with an empty value (rejected_<signal> = 0 and
                        /// `error_message` = "") is equivalent to it not being set/present. Senders
                        /// SHOULD interpret it the same way as in the full success case.
                        ///
                        /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                        pub r#partial_success: ExportLogsPartialSuccess,
                        /// Tracks presence of optional and message fields
                        pub _has: ExportLogsServiceResponse_::_Hazzer,
                    }
                    impl ::core::cmp::PartialEq for ExportLogsServiceResponse {
                        fn eq(&self, other: &Self) -> bool {
                            let mut ret = true;
                            ret
                                &= (self.r#partial_success() == other.r#partial_success());
                            ret
                        }
                    }
                    impl ExportLogsServiceResponse {
                        /// Return a reference to `partial_success` as an `Option`
                        #[inline]
                        pub fn r#partial_success(
                            &self,
                        ) -> ::core::option::Option<&ExportLogsPartialSuccess> {
                            self._has
                                .r#partial_success()
                                .then_some(&self.r#partial_success)
                        }
                        /// Set the value and presence of `partial_success`
                        #[inline]
                        pub fn set_partial_success(
                            &mut self,
                            value: ExportLogsPartialSuccess,
                        ) -> &mut Self {
                            self._has.set_partial_success();
                            self.r#partial_success = value.into();
                            self
                        }
                        /// Return a mutable reference to `partial_success` as an `Option`
                        #[inline]
                        pub fn mut_partial_success(
                            &mut self,
                        ) -> ::core::option::Option<&mut ExportLogsPartialSuccess> {
                            self._has
                                .r#partial_success()
                                .then_some(&mut self.r#partial_success)
                        }
                        /// Clear the presence of `partial_success`
                        #[inline]
                        pub fn clear_partial_success(&mut self) -> &mut Self {
                            self._has.clear_partial_success();
                            self
                        }
                        /// Take the value of `partial_success` and clear its presence
                        #[inline]
                        pub fn take_partial_success(
                            &mut self,
                        ) -> ::core::option::Option<ExportLogsPartialSuccess> {
                            let val = self
                                ._has
                                .r#partial_success()
                                .then(|| ::core::mem::take(&mut self.r#partial_success));
                            self._has.clear_partial_success();
                            val
                        }
                        /// Builder method that sets the value of `partial_success`. Useful for initializing the message.
                        #[inline]
                        pub fn init_partial_success(
                            mut self,
                            value: ExportLogsPartialSuccess,
                        ) -> Self {
                            self.set_partial_success(value);
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for ExportLogsServiceResponse {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#partial_success;
                                        {
                                            mut_ref.decode_len_delimited(decoder)?;
                                        };
                                        self._has.set_partial_success();
                                    }
                                    _ => {
                                        decoder.skip_wire_value(tag.wire_type())?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    impl ::micropb::MessageEncode for ExportLogsServiceResponse {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< ExportLogsPartialSuccess as
                                ::micropb::MessageEncode > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                if let ::core::option::Option::Some(val_ref) = self
                                    .r#partial_success()
                                {
                                    encoder.encode_varint32(10u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                if let ::core::option::Option::Some(val_ref) = self
                                    .r#partial_success()
                                {
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
                    /// Inner types for `ExportLogsServiceResponse`
                    pub mod ExportLogsServiceResponse_ {
                        /// Compact bitfield for tracking presence of optional and message fields
                        #[derive(Debug, Default, PartialEq, Clone, Copy)]
                        pub struct _Hazzer([u8; 1]);
                        impl _Hazzer {
                            /// New hazzer with all fields set to off
                            #[inline]
                            pub const fn _new() -> Self {
                                Self([0; 1])
                            }
                            /// Query presence of `partial_success`
                            #[inline]
                            pub const fn r#partial_success(&self) -> bool {
                                (self.0[0] & 1) != 0
                            }
                            /// Set presence of `partial_success`
                            #[inline]
                            pub const fn set_partial_success(&mut self) -> &mut Self {
                                let elem = &mut self.0[0];
                                *elem |= 1;
                                self
                            }
                            /// Clear presence of `partial_success`
                            #[inline]
                            pub const fn clear_partial_success(&mut self) -> &mut Self {
                                let elem = &mut self.0[0];
                                *elem &= !1;
                                self
                            }
                            /// Builder method that sets the presence of `partial_success`. Useful for initializing the Hazzer.
                            #[inline]
                            pub const fn init_partial_success(mut self) -> Self {
                                self.set_partial_success();
                                self
                            }
                        }
                    }
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct ExportLogsPartialSuccess {
                        /// The number of rejected log records.
                        ///
                        /// A `rejected_<signal>` field holding a `0` value indicates that the
                        /// request was fully accepted.
                        pub r#rejected_log_records: i64,
                        /// A developer-facing human-readable message in English. It should be used
                        /// either to explain why the server rejected parts of the data during a partial
                        /// success or to convey warnings/suggestions during a full success. The message
                        /// should offer guidance on how users can address such issues.
                        ///
                        /// error_message is an optional field. An error_message with an empty value
                        /// is equivalent to it not being set.
                        pub r#error_message: ::std::string::String,
                    }
                    impl ExportLogsPartialSuccess {
                        /// Return a reference to `rejected_log_records`
                        #[inline]
                        pub fn r#rejected_log_records(&self) -> &i64 {
                            &self.r#rejected_log_records
                        }
                        /// Return a mutable reference to `rejected_log_records`
                        #[inline]
                        pub fn mut_rejected_log_records(&mut self) -> &mut i64 {
                            &mut self.r#rejected_log_records
                        }
                        /// Set the value of `rejected_log_records`
                        #[inline]
                        pub fn set_rejected_log_records(
                            &mut self,
                            value: i64,
                        ) -> &mut Self {
                            self.r#rejected_log_records = value.into();
                            self
                        }
                        /// Builder method that sets the value of `rejected_log_records`. Useful for initializing the message.
                        #[inline]
                        pub fn init_rejected_log_records(mut self, value: i64) -> Self {
                            self.r#rejected_log_records = value.into();
                            self
                        }
                        /// Return a reference to `error_message`
                        #[inline]
                        pub fn r#error_message(&self) -> &::std::string::String {
                            &self.r#error_message
                        }
                        /// Return a mutable reference to `error_message`
                        #[inline]
                        pub fn mut_error_message(
                            &mut self,
                        ) -> &mut ::std::string::String {
                            &mut self.r#error_message
                        }
                        /// Set the value of `error_message`
                        #[inline]
                        pub fn set_error_message(
                            &mut self,
                            value: ::std::string::String,
                        ) -> &mut Self {
                            self.r#error_message = value.into();
                            self
                        }
                        /// Builder method that sets the value of `error_message`. Useful for initializing the message.
                        #[inline]
                        pub fn init_error_message(
                            mut self,
                            value: ::std::string::String,
                        ) -> Self {
                            self.r#error_message = value.into();
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for ExportLogsPartialSuccess {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#rejected_log_records;
                                        {
                                            let val = decoder.decode_int64()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    2u32 => {
                                        let mut_ref = &mut self.r#error_message;
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
                    impl ::micropb::MessageEncode for ExportLogsPartialSuccess {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(10usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.collector.logs.v1.ExportLogsPartialSuccess.error_message) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                let val_ref = &self.r#rejected_log_records;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(8u32)?;
                                    encoder.encode_int64(*val_ref as _)?;
                                }
                            }
                            {
                                let val_ref = &self.r#error_message;
                                if !val_ref.is_empty() {
                                    encoder.encode_varint32(18u32)?;
                                    encoder.encode_string(val_ref)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#rejected_log_records;
                                if *val_ref != 0 {
                                    size
                                        += 1usize + ::micropb::size::sizeof_int64(*val_ref as _);
                                }
                            }
                            {
                                let val_ref = &self.r#error_message;
                                if !val_ref.is_empty() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                            size
                        }
                    }
                }
            }
            pub mod metrics_ {
                pub mod v1_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct ExportMetricsServiceRequest {
                        /// An array of ResourceMetrics.
                        /// For data coming from a single resource this array will typically contain one
                        /// element. Intermediary nodes (such as OpenTelemetry Collector) that receive
                        /// data from multiple origins typically batch the data before forwarding further and
                        /// in that case this array will contain multiple elements.
                        pub r#resource_metrics: ::std::vec::Vec<
                            super::super::super::metrics_::v1_::ResourceMetrics,
                        >,
                    }
                    impl ExportMetricsServiceRequest {}
                    impl ::micropb::MessageDecode for ExportMetricsServiceRequest {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut val: super::super::super::metrics_::v1_::ResourceMetrics = ::core::default::Default::default();
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
                    impl ::micropb::MessageEncode for ExportMetricsServiceRequest {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::core::result::Result::<
                                usize,
                                &'static str,
                            >::Err(
                                "(.opentelemetry.proto.collector.metrics.v1.ExportMetricsServiceRequest.resource_metrics) unbounded vec",
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                for (i, val_ref) in self
                                    .r#resource_metrics
                                    .iter()
                                    .enumerate()
                                {
                                    encoder.encode_varint32(10u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                for (i, val_ref) in self
                                    .r#resource_metrics
                                    .iter()
                                    .enumerate()
                                {
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
                    #[derive(Debug, Default, Clone)]
                    pub struct ExportMetricsServiceResponse {
                        /// The details of a partially successful export request.
                        ///
                        /// If the request is only partially accepted
                        /// (i.e. when the server accepts only parts of the data and rejects the rest)
                        /// the server MUST initialize the `partial_success` field and MUST
                        /// set the `rejected_<signal>` with the number of items it rejected.
                        ///
                        /// Servers MAY also make use of the `partial_success` field to convey
                        /// warnings/suggestions to senders even when the request was fully accepted.
                        /// In such cases, the `rejected_<signal>` MUST have a value of `0` and
                        /// the `error_message` MUST be non-empty.
                        ///
                        /// A `partial_success` message with an empty value (rejected_<signal> = 0 and
                        /// `error_message` = "") is equivalent to it not being set/present. Senders
                        /// SHOULD interpret it the same way as in the full success case.
                        ///
                        /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                        pub r#partial_success: ExportMetricsPartialSuccess,
                        /// Tracks presence of optional and message fields
                        pub _has: ExportMetricsServiceResponse_::_Hazzer,
                    }
                    impl ::core::cmp::PartialEq for ExportMetricsServiceResponse {
                        fn eq(&self, other: &Self) -> bool {
                            let mut ret = true;
                            ret
                                &= (self.r#partial_success() == other.r#partial_success());
                            ret
                        }
                    }
                    impl ExportMetricsServiceResponse {
                        /// Return a reference to `partial_success` as an `Option`
                        #[inline]
                        pub fn r#partial_success(
                            &self,
                        ) -> ::core::option::Option<&ExportMetricsPartialSuccess> {
                            self._has
                                .r#partial_success()
                                .then_some(&self.r#partial_success)
                        }
                        /// Set the value and presence of `partial_success`
                        #[inline]
                        pub fn set_partial_success(
                            &mut self,
                            value: ExportMetricsPartialSuccess,
                        ) -> &mut Self {
                            self._has.set_partial_success();
                            self.r#partial_success = value.into();
                            self
                        }
                        /// Return a mutable reference to `partial_success` as an `Option`
                        #[inline]
                        pub fn mut_partial_success(
                            &mut self,
                        ) -> ::core::option::Option<&mut ExportMetricsPartialSuccess> {
                            self._has
                                .r#partial_success()
                                .then_some(&mut self.r#partial_success)
                        }
                        /// Clear the presence of `partial_success`
                        #[inline]
                        pub fn clear_partial_success(&mut self) -> &mut Self {
                            self._has.clear_partial_success();
                            self
                        }
                        /// Take the value of `partial_success` and clear its presence
                        #[inline]
                        pub fn take_partial_success(
                            &mut self,
                        ) -> ::core::option::Option<ExportMetricsPartialSuccess> {
                            let val = self
                                ._has
                                .r#partial_success()
                                .then(|| ::core::mem::take(&mut self.r#partial_success));
                            self._has.clear_partial_success();
                            val
                        }
                        /// Builder method that sets the value of `partial_success`. Useful for initializing the message.
                        #[inline]
                        pub fn init_partial_success(
                            mut self,
                            value: ExportMetricsPartialSuccess,
                        ) -> Self {
                            self.set_partial_success(value);
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for ExportMetricsServiceResponse {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#partial_success;
                                        {
                                            mut_ref.decode_len_delimited(decoder)?;
                                        };
                                        self._has.set_partial_success();
                                    }
                                    _ => {
                                        decoder.skip_wire_value(tag.wire_type())?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    impl ::micropb::MessageEncode for ExportMetricsServiceResponse {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< ExportMetricsPartialSuccess as
                                ::micropb::MessageEncode > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                if let ::core::option::Option::Some(val_ref) = self
                                    .r#partial_success()
                                {
                                    encoder.encode_varint32(10u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                if let ::core::option::Option::Some(val_ref) = self
                                    .r#partial_success()
                                {
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
                    /// Inner types for `ExportMetricsServiceResponse`
                    pub mod ExportMetricsServiceResponse_ {
                        /// Compact bitfield for tracking presence of optional and message fields
                        #[derive(Debug, Default, PartialEq, Clone, Copy)]
                        pub struct _Hazzer([u8; 1]);
                        impl _Hazzer {
                            /// New hazzer with all fields set to off
                            #[inline]
                            pub const fn _new() -> Self {
                                Self([0; 1])
                            }
                            /// Query presence of `partial_success`
                            #[inline]
                            pub const fn r#partial_success(&self) -> bool {
                                (self.0[0] & 1) != 0
                            }
                            /// Set presence of `partial_success`
                            #[inline]
                            pub const fn set_partial_success(&mut self) -> &mut Self {
                                let elem = &mut self.0[0];
                                *elem |= 1;
                                self
                            }
                            /// Clear presence of `partial_success`
                            #[inline]
                            pub const fn clear_partial_success(&mut self) -> &mut Self {
                                let elem = &mut self.0[0];
                                *elem &= !1;
                                self
                            }
                            /// Builder method that sets the presence of `partial_success`. Useful for initializing the Hazzer.
                            #[inline]
                            pub const fn init_partial_success(mut self) -> Self {
                                self.set_partial_success();
                                self
                            }
                        }
                    }
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct ExportMetricsPartialSuccess {
                        /// The number of rejected data points.
                        ///
                        /// A `rejected_<signal>` field holding a `0` value indicates that the
                        /// request was fully accepted.
                        pub r#rejected_data_points: i64,
                        /// A developer-facing human-readable message in English. It should be used
                        /// either to explain why the server rejected parts of the data during a partial
                        /// success or to convey warnings/suggestions during a full success. The message
                        /// should offer guidance on how users can address such issues.
                        ///
                        /// error_message is an optional field. An error_message with an empty value
                        /// is equivalent to it not being set.
                        pub r#error_message: ::std::string::String,
                    }
                    impl ExportMetricsPartialSuccess {
                        /// Return a reference to `rejected_data_points`
                        #[inline]
                        pub fn r#rejected_data_points(&self) -> &i64 {
                            &self.r#rejected_data_points
                        }
                        /// Return a mutable reference to `rejected_data_points`
                        #[inline]
                        pub fn mut_rejected_data_points(&mut self) -> &mut i64 {
                            &mut self.r#rejected_data_points
                        }
                        /// Set the value of `rejected_data_points`
                        #[inline]
                        pub fn set_rejected_data_points(
                            &mut self,
                            value: i64,
                        ) -> &mut Self {
                            self.r#rejected_data_points = value.into();
                            self
                        }
                        /// Builder method that sets the value of `rejected_data_points`. Useful for initializing the message.
                        #[inline]
                        pub fn init_rejected_data_points(mut self, value: i64) -> Self {
                            self.r#rejected_data_points = value.into();
                            self
                        }
                        /// Return a reference to `error_message`
                        #[inline]
                        pub fn r#error_message(&self) -> &::std::string::String {
                            &self.r#error_message
                        }
                        /// Return a mutable reference to `error_message`
                        #[inline]
                        pub fn mut_error_message(
                            &mut self,
                        ) -> &mut ::std::string::String {
                            &mut self.r#error_message
                        }
                        /// Set the value of `error_message`
                        #[inline]
                        pub fn set_error_message(
                            &mut self,
                            value: ::std::string::String,
                        ) -> &mut Self {
                            self.r#error_message = value.into();
                            self
                        }
                        /// Builder method that sets the value of `error_message`. Useful for initializing the message.
                        #[inline]
                        pub fn init_error_message(
                            mut self,
                            value: ::std::string::String,
                        ) -> Self {
                            self.r#error_message = value.into();
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for ExportMetricsPartialSuccess {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#rejected_data_points;
                                        {
                                            let val = decoder.decode_int64()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    2u32 => {
                                        let mut_ref = &mut self.r#error_message;
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
                    impl ::micropb::MessageEncode for ExportMetricsPartialSuccess {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(10usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.collector.metrics.v1.ExportMetricsPartialSuccess.error_message) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                let val_ref = &self.r#rejected_data_points;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(8u32)?;
                                    encoder.encode_int64(*val_ref as _)?;
                                }
                            }
                            {
                                let val_ref = &self.r#error_message;
                                if !val_ref.is_empty() {
                                    encoder.encode_varint32(18u32)?;
                                    encoder.encode_string(val_ref)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#rejected_data_points;
                                if *val_ref != 0 {
                                    size
                                        += 1usize + ::micropb::size::sizeof_int64(*val_ref as _);
                                }
                            }
                            {
                                let val_ref = &self.r#error_message;
                                if !val_ref.is_empty() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                            size
                        }
                    }
                }
            }
            pub mod trace_ {
                pub mod v1_ {
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct ExportTraceServiceRequest {
                        /// An array of ResourceSpans.
                        /// For data coming from a single resource this array will typically contain one
                        /// element. Intermediary nodes (such as OpenTelemetry Collector) that receive
                        /// data from multiple origins typically batch the data before forwarding further and
                        /// in that case this array will contain multiple elements.
                        pub r#resource_spans: ::std::vec::Vec<
                            super::super::super::trace_::v1_::ResourceSpans,
                        >,
                    }
                    impl ExportTraceServiceRequest {}
                    impl ::micropb::MessageDecode for ExportTraceServiceRequest {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut val: super::super::super::trace_::v1_::ResourceSpans = ::core::default::Default::default();
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
                    impl ::micropb::MessageEncode for ExportTraceServiceRequest {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::core::result::Result::<
                                usize,
                                &'static str,
                            >::Err(
                                "(.opentelemetry.proto.collector.trace.v1.ExportTraceServiceRequest.resource_spans) unbounded vec",
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                for (i, val_ref) in self.r#resource_spans.iter().enumerate()
                                {
                                    encoder.encode_varint32(10u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                for (i, val_ref) in self.r#resource_spans.iter().enumerate()
                                {
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
                    #[derive(Debug, Default, Clone)]
                    pub struct ExportTraceServiceResponse {
                        /// The details of a partially successful export request.
                        ///
                        /// If the request is only partially accepted
                        /// (i.e. when the server accepts only parts of the data and rejects the rest)
                        /// the server MUST initialize the `partial_success` field and MUST
                        /// set the `rejected_<signal>` with the number of items it rejected.
                        ///
                        /// Servers MAY also make use of the `partial_success` field to convey
                        /// warnings/suggestions to senders even when the request was fully accepted.
                        /// In such cases, the `rejected_<signal>` MUST have a value of `0` and
                        /// the `error_message` MUST be non-empty.
                        ///
                        /// A `partial_success` message with an empty value (rejected_<signal> = 0 and
                        /// `error_message` = "") is equivalent to it not being set/present. Senders
                        /// SHOULD interpret it the same way as in the full success case.
                        ///
                        /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                        pub r#partial_success: ExportTracePartialSuccess,
                        /// Tracks presence of optional and message fields
                        pub _has: ExportTraceServiceResponse_::_Hazzer,
                    }
                    impl ::core::cmp::PartialEq for ExportTraceServiceResponse {
                        fn eq(&self, other: &Self) -> bool {
                            let mut ret = true;
                            ret
                                &= (self.r#partial_success() == other.r#partial_success());
                            ret
                        }
                    }
                    impl ExportTraceServiceResponse {
                        /// Return a reference to `partial_success` as an `Option`
                        #[inline]
                        pub fn r#partial_success(
                            &self,
                        ) -> ::core::option::Option<&ExportTracePartialSuccess> {
                            self._has
                                .r#partial_success()
                                .then_some(&self.r#partial_success)
                        }
                        /// Set the value and presence of `partial_success`
                        #[inline]
                        pub fn set_partial_success(
                            &mut self,
                            value: ExportTracePartialSuccess,
                        ) -> &mut Self {
                            self._has.set_partial_success();
                            self.r#partial_success = value.into();
                            self
                        }
                        /// Return a mutable reference to `partial_success` as an `Option`
                        #[inline]
                        pub fn mut_partial_success(
                            &mut self,
                        ) -> ::core::option::Option<&mut ExportTracePartialSuccess> {
                            self._has
                                .r#partial_success()
                                .then_some(&mut self.r#partial_success)
                        }
                        /// Clear the presence of `partial_success`
                        #[inline]
                        pub fn clear_partial_success(&mut self) -> &mut Self {
                            self._has.clear_partial_success();
                            self
                        }
                        /// Take the value of `partial_success` and clear its presence
                        #[inline]
                        pub fn take_partial_success(
                            &mut self,
                        ) -> ::core::option::Option<ExportTracePartialSuccess> {
                            let val = self
                                ._has
                                .r#partial_success()
                                .then(|| ::core::mem::take(&mut self.r#partial_success));
                            self._has.clear_partial_success();
                            val
                        }
                        /// Builder method that sets the value of `partial_success`. Useful for initializing the message.
                        #[inline]
                        pub fn init_partial_success(
                            mut self,
                            value: ExportTracePartialSuccess,
                        ) -> Self {
                            self.set_partial_success(value);
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for ExportTraceServiceResponse {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#partial_success;
                                        {
                                            mut_ref.decode_len_delimited(decoder)?;
                                        };
                                        self._has.set_partial_success();
                                    }
                                    _ => {
                                        decoder.skip_wire_value(tag.wire_type())?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    impl ::micropb::MessageEncode for ExportTraceServiceResponse {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< ExportTracePartialSuccess as
                                ::micropb::MessageEncode > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                if let ::core::option::Option::Some(val_ref) = self
                                    .r#partial_success()
                                {
                                    encoder.encode_varint32(10u32)?;
                                    val_ref.encode_len_delimited(encoder)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                if let ::core::option::Option::Some(val_ref) = self
                                    .r#partial_success()
                                {
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
                    /// Inner types for `ExportTraceServiceResponse`
                    pub mod ExportTraceServiceResponse_ {
                        /// Compact bitfield for tracking presence of optional and message fields
                        #[derive(Debug, Default, PartialEq, Clone, Copy)]
                        pub struct _Hazzer([u8; 1]);
                        impl _Hazzer {
                            /// New hazzer with all fields set to off
                            #[inline]
                            pub const fn _new() -> Self {
                                Self([0; 1])
                            }
                            /// Query presence of `partial_success`
                            #[inline]
                            pub const fn r#partial_success(&self) -> bool {
                                (self.0[0] & 1) != 0
                            }
                            /// Set presence of `partial_success`
                            #[inline]
                            pub const fn set_partial_success(&mut self) -> &mut Self {
                                let elem = &mut self.0[0];
                                *elem |= 1;
                                self
                            }
                            /// Clear presence of `partial_success`
                            #[inline]
                            pub const fn clear_partial_success(&mut self) -> &mut Self {
                                let elem = &mut self.0[0];
                                *elem &= !1;
                                self
                            }
                            /// Builder method that sets the presence of `partial_success`. Useful for initializing the Hazzer.
                            #[inline]
                            pub const fn init_partial_success(mut self) -> Self {
                                self.set_partial_success();
                                self
                            }
                        }
                    }
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct ExportTracePartialSuccess {
                        /// The number of rejected spans.
                        ///
                        /// A `rejected_<signal>` field holding a `0` value indicates that the
                        /// request was fully accepted.
                        pub r#rejected_spans: i64,
                        /// A developer-facing human-readable message in English. It should be used
                        /// either to explain why the server rejected parts of the data during a partial
                        /// success or to convey warnings/suggestions during a full success. The message
                        /// should offer guidance on how users can address such issues.
                        ///
                        /// error_message is an optional field. An error_message with an empty value
                        /// is equivalent to it not being set.
                        pub r#error_message: ::std::string::String,
                    }
                    impl ExportTracePartialSuccess {
                        /// Return a reference to `rejected_spans`
                        #[inline]
                        pub fn r#rejected_spans(&self) -> &i64 {
                            &self.r#rejected_spans
                        }
                        /// Return a mutable reference to `rejected_spans`
                        #[inline]
                        pub fn mut_rejected_spans(&mut self) -> &mut i64 {
                            &mut self.r#rejected_spans
                        }
                        /// Set the value of `rejected_spans`
                        #[inline]
                        pub fn set_rejected_spans(&mut self, value: i64) -> &mut Self {
                            self.r#rejected_spans = value.into();
                            self
                        }
                        /// Builder method that sets the value of `rejected_spans`. Useful for initializing the message.
                        #[inline]
                        pub fn init_rejected_spans(mut self, value: i64) -> Self {
                            self.r#rejected_spans = value.into();
                            self
                        }
                        /// Return a reference to `error_message`
                        #[inline]
                        pub fn r#error_message(&self) -> &::std::string::String {
                            &self.r#error_message
                        }
                        /// Return a mutable reference to `error_message`
                        #[inline]
                        pub fn mut_error_message(
                            &mut self,
                        ) -> &mut ::std::string::String {
                            &mut self.r#error_message
                        }
                        /// Set the value of `error_message`
                        #[inline]
                        pub fn set_error_message(
                            &mut self,
                            value: ::std::string::String,
                        ) -> &mut Self {
                            self.r#error_message = value.into();
                            self
                        }
                        /// Builder method that sets the value of `error_message`. Useful for initializing the message.
                        #[inline]
                        pub fn init_error_message(
                            mut self,
                            value: ::std::string::String,
                        ) -> Self {
                            self.r#error_message = value.into();
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for ExportTracePartialSuccess {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
                            let before = decoder.bytes_read();
                            while decoder.bytes_read() - before < len {
                                let tag = decoder.decode_tag()?;
                                match tag.field_num() {
                                    0 => return Err(::micropb::DecodeError::ZeroField),
                                    1u32 => {
                                        let mut_ref = &mut self.r#rejected_spans;
                                        {
                                            let val = decoder.decode_int64()?;
                                            let val_ref = &val;
                                            if *val_ref != 0 {
                                                *mut_ref = val as _;
                                            }
                                        };
                                    }
                                    2u32 => {
                                        let mut_ref = &mut self.r#error_message;
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
                    impl ::micropb::MessageEncode for ExportTracePartialSuccess {
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(10usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.collector.trace.v1.ExportTracePartialSuccess.error_message) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                let val_ref = &self.r#rejected_spans;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(8u32)?;
                                    encoder.encode_int64(*val_ref as _)?;
                                }
                            }
                            {
                                let val_ref = &self.r#error_message;
                                if !val_ref.is_empty() {
                                    encoder.encode_varint32(18u32)?;
                                    encoder.encode_string(val_ref)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#rejected_spans;
                                if *val_ref != 0 {
                                    size
                                        += 1usize + ::micropb::size::sizeof_int64(*val_ref as _);
                                }
                            }
                            {
                                let val_ref = &self.r#error_message;
                                if !val_ref.is_empty() {
                                    size
                                        += 1usize
                                            + ::micropb::size::sizeof_len_record(val_ref.len());
                                }
                            }
                            size
                        }
                    }
                }
            }
        }
        pub mod metrics_ {
            pub mod v1_ {
                /// MetricsData represents the metrics data that can be stored in a persistent
                /// storage, OR can be embedded by other protocols that transfer OTLP metrics
                /// data but do not implement the OTLP protocol.
                ///
                /// MetricsData
                /// └─── ResourceMetrics
                ///   ├── Resource
                ///   ├── SchemaURL
                ///   └── ScopeMetrics
                ///      ├── Scope
                ///      ├── SchemaURL
                ///      └── Metric
                ///         ├── Name
                ///         ├── Description
                ///         ├── Unit
                ///         └── data
                ///            ├── Gauge
                ///            ├── Sum
                ///            ├── Histogram
                ///            ├── ExponentialHistogram
                ///            └── Summary
                ///
                /// The main difference between this message and collector protocol is that
                /// in this message there will not be any "control" or "metadata" specific to
                /// OTLP protocol.
                ///
                /// When new fields are added into this message, the OTLP request MUST be updated
                /// as well.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct MetricsData {
                    /// An array of ResourceMetrics.
                    /// For data coming from a single resource this array will typically contain
                    /// one element. Intermediary nodes that receive data from multiple origins
                    /// typically batch the data before forwarding further and in that case this
                    /// array will contain multiple elements.
                    pub r#resource_metrics: ::std::vec::Vec<ResourceMetrics>,
                }
                impl MetricsData {}
                impl ::micropb::MessageDecode for MetricsData {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.MetricsData.resource_metrics) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self
                                .r#resource_metrics
                                .iter()
                                .enumerate()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self
                                .r#resource_metrics
                                .iter()
                                .enumerate()
                            {
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
                /// A collection of ScopeMetrics from a Resource.
                #[derive(Debug, Default, Clone)]
                pub struct ResourceMetrics {
                    /// The resource for the metrics in this message.
                    /// If this field is not set then no resource info is known.
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#resource: super::super::resource_::v1_::Resource,
                    /// A list of metrics that originate from a resource.
                    pub r#scope_metrics: ::std::vec::Vec<ScopeMetrics>,
                    /// The Schema URL, if known. This is the identifier of the Schema that the resource data
                    /// is recorded in. Notably, the last part of the URL path is the version number of the
                    /// schema: http[s]://server[:port]/path/<version>. To learn more about Schema URL see
                    /// https://opentelemetry.io/docs/specs/otel/schemas/#schema-url
                    /// This schema_url applies to the data in the "resource" field. It does not apply
                    /// to the data in the "scope_metrics" field which have their own schema_url field.
                    pub r#schema_url: ::std::string::String,
                    /// Tracks presence of optional and message fields
                    pub _has: ResourceMetrics_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for ResourceMetrics {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#resource() == other.r#resource());
                        ret &= (self.r#scope_metrics == other.r#scope_metrics);
                        ret &= (self.r#schema_url == other.r#schema_url);
                        ret
                    }
                }
                impl ResourceMetrics {
                    /// Return a reference to `resource` as an `Option`
                    #[inline]
                    pub fn r#resource(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&self.r#resource)
                    }
                    /// Set the value and presence of `resource`
                    #[inline]
                    pub fn set_resource(
                        &mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) -> &mut Self {
                        self._has.set_resource();
                        self.r#resource = value.into();
                        self
                    }
                    /// Return a mutable reference to `resource` as an `Option`
                    #[inline]
                    pub fn mut_resource(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&mut self.r#resource)
                    }
                    /// Clear the presence of `resource`
                    #[inline]
                    pub fn clear_resource(&mut self) -> &mut Self {
                        self._has.clear_resource();
                        self
                    }
                    /// Take the value of `resource` and clear its presence
                    #[inline]
                    pub fn take_resource(
                        &mut self,
                    ) -> ::core::option::Option<super::super::resource_::v1_::Resource> {
                        let val = self
                            ._has
                            .r#resource()
                            .then(|| ::core::mem::take(&mut self.r#resource));
                        self._has.clear_resource();
                        val
                    }
                    /// Builder method that sets the value of `resource`. Useful for initializing the message.
                    #[inline]
                    pub fn init_resource(
                        mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) -> Self {
                        self.set_resource(value);
                        self
                    }
                    /// Return a reference to `schema_url`
                    #[inline]
                    pub fn r#schema_url(&self) -> &::std::string::String {
                        &self.r#schema_url
                    }
                    /// Return a mutable reference to `schema_url`
                    #[inline]
                    pub fn mut_schema_url(&mut self) -> &mut ::std::string::String {
                        &mut self.r#schema_url
                    }
                    /// Set the value of `schema_url`
                    #[inline]
                    pub fn set_schema_url(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Builder method that sets the value of `schema_url`. Useful for initializing the message.
                    #[inline]
                    pub fn init_schema_url(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#schema_url = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ResourceMetrics {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            super::super::resource_::v1_::Resource as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.ResourceMetrics.scope_metrics) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.metrics.v1.ResourceMetrics.schema_url) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#resource()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#scope_metrics.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#resource()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for (i, val_ref) in self.r#scope_metrics.iter().enumerate() {
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
                /// Inner types for `ResourceMetrics`
                pub mod ResourceMetrics_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `resource`
                        #[inline]
                        pub const fn r#resource(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `resource`
                        #[inline]
                        pub const fn set_resource(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `resource`
                        #[inline]
                        pub const fn clear_resource(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `resource`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_resource(mut self) -> Self {
                            self.set_resource();
                            self
                        }
                    }
                }
                /// A collection of Metrics produced by an Scope.
                #[derive(Debug, Default, Clone)]
                pub struct ScopeMetrics {
                    /// The instrumentation scope information for the metrics in this message.
                    /// Semantically when InstrumentationScope isn't set, it is equivalent with
                    /// an empty instrumentation scope name (unknown).
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#scope: super::super::common_::v1_::InstrumentationScope,
                    /// A list of metrics that originate from an instrumentation library.
                    pub r#metrics: ::std::vec::Vec<Metric>,
                    /// The Schema URL, if known. This is the identifier of the Schema that the metric data
                    /// is recorded in. Notably, the last part of the URL path is the version number of the
                    /// schema: http[s]://server[:port]/path/<version>. To learn more about Schema URL see
                    /// https://opentelemetry.io/docs/specs/otel/schemas/#schema-url
                    /// This schema_url applies to the data in the "scope" field and all metrics in the
                    /// "metrics" field.
                    pub r#schema_url: ::std::string::String,
                    /// Tracks presence of optional and message fields
                    pub _has: ScopeMetrics_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for ScopeMetrics {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#scope() == other.r#scope());
                        ret &= (self.r#metrics == other.r#metrics);
                        ret &= (self.r#schema_url == other.r#schema_url);
                        ret
                    }
                }
                impl ScopeMetrics {
                    /// Return a reference to `scope` as an `Option`
                    #[inline]
                    pub fn r#scope(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&self.r#scope)
                    }
                    /// Set the value and presence of `scope`
                    #[inline]
                    pub fn set_scope(
                        &mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) -> &mut Self {
                        self._has.set_scope();
                        self.r#scope = value.into();
                        self
                    }
                    /// Return a mutable reference to `scope` as an `Option`
                    #[inline]
                    pub fn mut_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&mut self.r#scope)
                    }
                    /// Clear the presence of `scope`
                    #[inline]
                    pub fn clear_scope(&mut self) -> &mut Self {
                        self._has.clear_scope();
                        self
                    }
                    /// Take the value of `scope` and clear its presence
                    #[inline]
                    pub fn take_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        super::super::common_::v1_::InstrumentationScope,
                    > {
                        let val = self
                            ._has
                            .r#scope()
                            .then(|| ::core::mem::take(&mut self.r#scope));
                        self._has.clear_scope();
                        val
                    }
                    /// Builder method that sets the value of `scope`. Useful for initializing the message.
                    #[inline]
                    pub fn init_scope(
                        mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) -> Self {
                        self.set_scope(value);
                        self
                    }
                    /// Return a reference to `schema_url`
                    #[inline]
                    pub fn r#schema_url(&self) -> &::std::string::String {
                        &self.r#schema_url
                    }
                    /// Return a mutable reference to `schema_url`
                    #[inline]
                    pub fn mut_schema_url(&mut self) -> &mut ::std::string::String {
                        &mut self.r#schema_url
                    }
                    /// Set the value of `schema_url`
                    #[inline]
                    pub fn set_schema_url(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Builder method that sets the value of `schema_url`. Useful for initializing the message.
                    #[inline]
                    pub fn init_schema_url(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#schema_url = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ScopeMetrics {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            super::super::common_::v1_::InstrumentationScope as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.ScopeMetrics.metrics) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.metrics.v1.ScopeMetrics.schema_url) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#scope()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#metrics.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#scope()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for (i, val_ref) in self.r#metrics.iter().enumerate() {
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
                /// Inner types for `ScopeMetrics`
                pub mod ScopeMetrics_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `scope`
                        #[inline]
                        pub const fn r#scope(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `scope`
                        #[inline]
                        pub const fn set_scope(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `scope`
                        #[inline]
                        pub const fn clear_scope(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `scope`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_scope(mut self) -> Self {
                            self.set_scope();
                            self
                        }
                    }
                }
                /// Defines a Metric which has one or more timeseries.  The following is a
                /// brief summary of the Metric data model.  For more details, see:
                ///
                ///   https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/data-model.md
                ///
                /// The data model and relation between entities is shown in the
                /// diagram below. Here, "DataPoint" is the term used to refer to any
                /// one of the specific data point value types, and "points" is the term used
                /// to refer to any one of the lists of points contained in the Metric.
                ///
                /// - Metric is composed of a metadata and data.
                /// - Metadata part contains a name, description, unit.
                /// - Data is one of the possible types (Sum, Gauge, Histogram, Summary).
                /// - DataPoint contains timestamps, attributes, and one of the possible value type
                ///   fields.
                ///
                ///    Metric
                ///  +------------+
                ///  |name        |
                ///  |description |
                ///  |unit        |     +------------------------------------+
                ///  |data        |---> |Gauge, Sum, Histogram, Summary, ... |
                ///  +------------+     +------------------------------------+
                ///
                ///    Data [One of Gauge, Sum, Histogram, Summary, ...]
                ///  +-----------+
                ///  |...        |  // Metadata about the Data.
                ///  |points     |--+
                ///  +-----------+  |
                ///                 |      +---------------------------+
                ///                 |      |DataPoint 1                |
                ///                 v      |+------+------+   +------+ |
                ///              +-----+   ||label |label |...|label | |
                ///              |  1  |-->||value1|value2|...|valueN| |
                ///              +-----+   |+------+------+   +------+ |
                ///              |  .  |   |+-----+                    |
                ///              |  .  |   ||value|                    |
                ///              |  .  |   |+-----+                    |
                ///              |  .  |   +---------------------------+
                ///              |  .  |                   .
                ///              |  .  |                   .
                ///              |  .  |                   .
                ///              |  .  |   +---------------------------+
                ///              |  .  |   |DataPoint M                |
                ///              +-----+   |+------+------+   +------+ |
                ///              |  M  |-->||label |label |...|label | |
                ///              +-----+   ||value1|value2|...|valueN| |
                ///                        |+------+------+   +------+ |
                ///                        |+-----+                    |
                ///                        ||value|                    |
                ///                        |+-----+                    |
                ///                        +---------------------------+
                ///
                /// Each distinct type of DataPoint represents the output of a specific
                /// aggregation function, the result of applying the DataPoint's
                /// associated function of to one or more measurements.
                ///
                /// All DataPoint types have three common fields:
                /// - Attributes includes key-value pairs associated with the data point
                /// - TimeUnixNano is required, set to the end time of the aggregation
                /// - StartTimeUnixNano is optional, but strongly encouraged for DataPoints
                ///   having an AggregationTemporality field, as discussed below.
                ///
                /// Both TimeUnixNano and StartTimeUnixNano values are expressed as
                /// UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
                ///
                /// # TimeUnixNano
                ///
                /// This field is required, having consistent interpretation across
                /// DataPoint types.  TimeUnixNano is the moment corresponding to when
                /// the data point's aggregate value was captured.
                ///
                /// Data points with the 0 value for TimeUnixNano SHOULD be rejected
                /// by consumers.
                ///
                /// # StartTimeUnixNano
                ///
                /// StartTimeUnixNano in general allows detecting when a sequence of
                /// observations is unbroken.  This field indicates to consumers the
                /// start time for points with cumulative and delta
                /// AggregationTemporality, and it should be included whenever possible
                /// to support correct rate calculation.  Although it may be omitted
                /// when the start time is truly unknown, setting StartTimeUnixNano is
                /// strongly encouraged.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Metric {
                    /// The name of the metric.
                    pub r#name: ::std::string::String,
                    /// A description of the metric, which can be used in documentation.
                    pub r#description: ::std::string::String,
                    /// The unit in which the metric value is reported. Follows the format
                    /// described by https://unitsofmeasure.org/ucum.html.
                    pub r#unit: ::std::string::String,
                    /// Additional metadata attributes that describe the metric. [Optional].
                    /// Attributes are non-identifying.
                    /// Consumers SHOULD NOT need to be aware of these attributes.
                    /// These attributes MAY be used to encode information allowing
                    /// for lossless roundtrip translation to / from another data model.
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#metadata: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// Data determines the aggregation type (if any) of the metric, what is the
                    /// reported value type for the data points, as well as the relatationship to
                    /// the time interval over which they are reported.
                    pub r#data: ::core::option::Option<Metric_::Data>,
                }
                impl Metric {
                    /// Return a reference to `name`
                    #[inline]
                    pub fn r#name(&self) -> &::std::string::String {
                        &self.r#name
                    }
                    /// Return a mutable reference to `name`
                    #[inline]
                    pub fn mut_name(&mut self) -> &mut ::std::string::String {
                        &mut self.r#name
                    }
                    /// Set the value of `name`
                    #[inline]
                    pub fn set_name(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#name = value.into();
                        self
                    }
                    /// Builder method that sets the value of `name`. Useful for initializing the message.
                    #[inline]
                    pub fn init_name(mut self, value: ::std::string::String) -> Self {
                        self.r#name = value.into();
                        self
                    }
                    /// Return a reference to `description`
                    #[inline]
                    pub fn r#description(&self) -> &::std::string::String {
                        &self.r#description
                    }
                    /// Return a mutable reference to `description`
                    #[inline]
                    pub fn mut_description(&mut self) -> &mut ::std::string::String {
                        &mut self.r#description
                    }
                    /// Set the value of `description`
                    #[inline]
                    pub fn set_description(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#description = value.into();
                        self
                    }
                    /// Builder method that sets the value of `description`. Useful for initializing the message.
                    #[inline]
                    pub fn init_description(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#description = value.into();
                        self
                    }
                    /// Return a reference to `unit`
                    #[inline]
                    pub fn r#unit(&self) -> &::std::string::String {
                        &self.r#unit
                    }
                    /// Return a mutable reference to `unit`
                    #[inline]
                    pub fn mut_unit(&mut self) -> &mut ::std::string::String {
                        &mut self.r#unit
                    }
                    /// Set the value of `unit`
                    #[inline]
                    pub fn set_unit(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#unit = value.into();
                        self
                    }
                    /// Builder method that sets the value of `unit`. Useful for initializing the message.
                    #[inline]
                    pub fn init_unit(mut self, value: ::std::string::String) -> Self {
                        self.r#unit = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for Metric {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.metrics.v1.Metric.name) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.metrics.v1.Metric.description) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.metrics.v1.Metric.unit) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.Metric.metadata) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match 'oneof: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< Gauge as ::micropb::MessageEncode >
                                ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< Sum as ::micropb::MessageEncode >
                                ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< Histogram as
                                ::micropb::MessageEncode > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< ExponentialHistogram as
                                ::micropb::MessageEncode > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::micropb::const_map!(< Summary as ::micropb::MessageEncode
                                > ::MAX_SIZE, | size |
                                ::micropb::size::sizeof_len_record(size)), | size | size +
                                1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        } {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
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
                            for (i, val_ref) in self.r#metadata.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
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
                            for (i, val_ref) in self.r#metadata.iter().enumerate() {
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
                /// Inner types for `Metric`
                pub mod Metric_ {
                    /// Data determines the aggregation type (if any) of the metric, what is the
                    /// reported value type for the data points, as well as the relatationship to
                    /// the time interval over which they are reported.
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Data {
                        Gauge(super::Gauge),
                        Sum(super::Sum),
                        Histogram(super::Histogram),
                        ExponentialHistogram(super::ExponentialHistogram),
                        Summary(super::Summary),
                    }
                }
                /// Gauge represents the type of a scalar metric that always exports the
                /// "current value" for every data point. It should be used for an "unknown"
                /// aggregation.
                ///
                /// A Gauge does not support different aggregation temporalities. Given the
                /// aggregation is unknown, points cannot be combined using the same
                /// aggregation, regardless of aggregation temporalities. Therefore,
                /// AggregationTemporality is not included. Consequently, this also means
                /// "StartTimeUnixNano" is ignored for all data points.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Gauge {
                    /// The time series data points.
                    /// Note: Multiple time series may be included (same timestamp, different attributes).
                    pub r#data_points: ::std::vec::Vec<NumberDataPoint>,
                }
                impl Gauge {}
                impl ::micropb::MessageDecode for Gauge {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.Gauge.data_points) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                /// Sum represents the type of a scalar metric that is calculated as a sum of all
                /// reported measurements over a time interval.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Sum {
                    /// The time series data points.
                    /// Note: Multiple time series may be included (same timestamp, different attributes).
                    pub r#data_points: ::std::vec::Vec<NumberDataPoint>,
                    /// aggregation_temporality describes if the aggregator reports delta changes
                    /// since last report time, or cumulative changes since a fixed start time.
                    pub r#aggregation_temporality: AggregationTemporality,
                    /// Represents whether the sum is monotonic.
                    pub r#is_monotonic: bool,
                }
                impl Sum {
                    /// Return a reference to `aggregation_temporality`
                    #[inline]
                    pub fn r#aggregation_temporality(&self) -> &AggregationTemporality {
                        &self.r#aggregation_temporality
                    }
                    /// Return a mutable reference to `aggregation_temporality`
                    #[inline]
                    pub fn mut_aggregation_temporality(
                        &mut self,
                    ) -> &mut AggregationTemporality {
                        &mut self.r#aggregation_temporality
                    }
                    /// Set the value of `aggregation_temporality`
                    #[inline]
                    pub fn set_aggregation_temporality(
                        &mut self,
                        value: AggregationTemporality,
                    ) -> &mut Self {
                        self.r#aggregation_temporality = value.into();
                        self
                    }
                    /// Builder method that sets the value of `aggregation_temporality`. Useful for initializing the message.
                    #[inline]
                    pub fn init_aggregation_temporality(
                        mut self,
                        value: AggregationTemporality,
                    ) -> Self {
                        self.r#aggregation_temporality = value.into();
                        self
                    }
                    /// Return a reference to `is_monotonic`
                    #[inline]
                    pub fn r#is_monotonic(&self) -> &bool {
                        &self.r#is_monotonic
                    }
                    /// Return a mutable reference to `is_monotonic`
                    #[inline]
                    pub fn mut_is_monotonic(&mut self) -> &mut bool {
                        &mut self.r#is_monotonic
                    }
                    /// Set the value of `is_monotonic`
                    #[inline]
                    pub fn set_is_monotonic(&mut self, value: bool) -> &mut Self {
                        self.r#is_monotonic = value.into();
                        self
                    }
                    /// Builder method that sets the value of `is_monotonic`. Useful for initializing the message.
                    #[inline]
                    pub fn init_is_monotonic(mut self, value: bool) -> Self {
                        self.r#is_monotonic = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for Sum {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.Sum.data_points) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(AggregationTemporality::_MAX_SIZE),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(1usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                /// Histogram represents the type of a metric that is calculated by aggregating
                /// as a Histogram of all reported measurements over a time interval.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Histogram {
                    /// The time series data points.
                    /// Note: Multiple time series may be included (same timestamp, different attributes).
                    pub r#data_points: ::std::vec::Vec<HistogramDataPoint>,
                    /// aggregation_temporality describes if the aggregator reports delta changes
                    /// since last report time, or cumulative changes since a fixed start time.
                    pub r#aggregation_temporality: AggregationTemporality,
                }
                impl Histogram {
                    /// Return a reference to `aggregation_temporality`
                    #[inline]
                    pub fn r#aggregation_temporality(&self) -> &AggregationTemporality {
                        &self.r#aggregation_temporality
                    }
                    /// Return a mutable reference to `aggregation_temporality`
                    #[inline]
                    pub fn mut_aggregation_temporality(
                        &mut self,
                    ) -> &mut AggregationTemporality {
                        &mut self.r#aggregation_temporality
                    }
                    /// Set the value of `aggregation_temporality`
                    #[inline]
                    pub fn set_aggregation_temporality(
                        &mut self,
                        value: AggregationTemporality,
                    ) -> &mut Self {
                        self.r#aggregation_temporality = value.into();
                        self
                    }
                    /// Builder method that sets the value of `aggregation_temporality`. Useful for initializing the message.
                    #[inline]
                    pub fn init_aggregation_temporality(
                        mut self,
                        value: AggregationTemporality,
                    ) -> Self {
                        self.r#aggregation_temporality = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for Histogram {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.Histogram.data_points) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(AggregationTemporality::_MAX_SIZE),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                /// ExponentialHistogram represents the type of a metric that is calculated by aggregating
                /// as a ExponentialHistogram of all reported double measurements over a time interval.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct ExponentialHistogram {
                    /// The time series data points.
                    /// Note: Multiple time series may be included (same timestamp, different attributes).
                    pub r#data_points: ::std::vec::Vec<ExponentialHistogramDataPoint>,
                    /// aggregation_temporality describes if the aggregator reports delta changes
                    /// since last report time, or cumulative changes since a fixed start time.
                    pub r#aggregation_temporality: AggregationTemporality,
                }
                impl ExponentialHistogram {
                    /// Return a reference to `aggregation_temporality`
                    #[inline]
                    pub fn r#aggregation_temporality(&self) -> &AggregationTemporality {
                        &self.r#aggregation_temporality
                    }
                    /// Return a mutable reference to `aggregation_temporality`
                    #[inline]
                    pub fn mut_aggregation_temporality(
                        &mut self,
                    ) -> &mut AggregationTemporality {
                        &mut self.r#aggregation_temporality
                    }
                    /// Set the value of `aggregation_temporality`
                    #[inline]
                    pub fn set_aggregation_temporality(
                        &mut self,
                        value: AggregationTemporality,
                    ) -> &mut Self {
                        self.r#aggregation_temporality = value.into();
                        self
                    }
                    /// Builder method that sets the value of `aggregation_temporality`. Useful for initializing the message.
                    #[inline]
                    pub fn init_aggregation_temporality(
                        mut self,
                        value: AggregationTemporality,
                    ) -> Self {
                        self.r#aggregation_temporality = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ExponentialHistogram {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.ExponentialHistogram.data_points) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(AggregationTemporality::_MAX_SIZE),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                /// Summary metric data are used to convey quantile summaries,
                /// a Prometheus (see: https://prometheus.io/docs/concepts/metric_types/#summary)
                /// and OpenMetrics (see: https://github.com/prometheus/OpenMetrics/blob/4dbf6075567ab43296eed941037c12951faafb92/protos/prometheus.proto#L45)
                /// data type. These data points cannot always be merged in a meaningful way.
                /// While they can be useful in some applications, histogram data points are
                /// recommended for new applications.
                /// Summary metrics do not have an aggregation temporality field. This is
                /// because the count and sum fields of a SummaryDataPoint are assumed to be
                /// cumulative values.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Summary {
                    /// The time series data points.
                    /// Note: Multiple time series may be included (same timestamp, different attributes).
                    pub r#data_points: ::std::vec::Vec<SummaryDataPoint>,
                }
                impl Summary {}
                impl ::micropb::MessageDecode for Summary {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.Summary.data_points) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#data_points.iter().enumerate() {
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
                /// NumberDataPoint is a single data point in a timeseries that describes the
                /// time-varying scalar value of a metric.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct NumberDataPoint {
                    /// The set of key/value pairs that uniquely identify the timeseries from
                    /// where this point belongs. The list may be empty (may contain 0 elements).
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// StartTimeUnixNano is optional but strongly encouraged, see the
                    /// the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#start_time_unix_nano: u64,
                    /// TimeUnixNano is required, see the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#time_unix_nano: u64,
                    /// (Optional) List of exemplars collected from
                    /// measurements that were used to form the data point
                    pub r#exemplars: ::std::vec::Vec<Exemplar>,
                    /// Flags that apply to this specific data point.  See DataPointFlags
                    /// for the available flags and their meaning.
                    pub r#flags: u32,
                    /// The value itself.  A point is considered invalid when one of the recognized
                    /// value fields is not present inside this oneof.
                    pub r#value: ::core::option::Option<NumberDataPoint_::Value>,
                }
                impl NumberDataPoint {
                    /// Return a reference to `start_time_unix_nano`
                    #[inline]
                    pub fn r#start_time_unix_nano(&self) -> &u64 {
                        &self.r#start_time_unix_nano
                    }
                    /// Return a mutable reference to `start_time_unix_nano`
                    #[inline]
                    pub fn mut_start_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#start_time_unix_nano
                    }
                    /// Set the value of `start_time_unix_nano`
                    #[inline]
                    pub fn set_start_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `start_time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_start_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `time_unix_nano`
                    #[inline]
                    pub fn r#time_unix_nano(&self) -> &u64 {
                        &self.r#time_unix_nano
                    }
                    /// Return a mutable reference to `time_unix_nano`
                    #[inline]
                    pub fn mut_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#time_unix_nano
                    }
                    /// Set the value of `time_unix_nano`
                    #[inline]
                    pub fn set_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `flags`
                    #[inline]
                    pub fn r#flags(&self) -> &u32 {
                        &self.r#flags
                    }
                    /// Return a mutable reference to `flags`
                    #[inline]
                    pub fn mut_flags(&mut self) -> &mut u32 {
                        &mut self.r#flags
                    }
                    /// Set the value of `flags`
                    #[inline]
                    pub fn set_flags(&mut self, value: u32) -> &mut Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Builder method that sets the value of `flags`. Useful for initializing the message.
                    #[inline]
                    pub fn init_flags(mut self, value: u32) -> Self {
                        self.r#flags = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for NumberDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.NumberDataPoint.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.NumberDataPoint.exemplars) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match 'oneof: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        } {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            for (i, val_ref) in self.r#exemplars.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            for (i, val_ref) in self.r#exemplars.iter().enumerate() {
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
                /// Inner types for `NumberDataPoint`
                pub mod NumberDataPoint_ {
                    /// The value itself.  A point is considered invalid when one of the recognized
                    /// value fields is not present inside this oneof.
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Value {
                        AsDouble(f64),
                        AsInt(i64),
                    }
                }
                /// HistogramDataPoint is a single data point in a timeseries that describes the
                /// time-varying values of a Histogram. A Histogram contains summary statistics
                /// for a population of values, it may optionally contain the distribution of
                /// those values across a set of buckets.
                ///
                /// If the histogram contains the distribution of values, then both
                /// "explicit_bounds" and "bucket counts" fields must be defined.
                /// If the histogram does not contain the distribution of values, then both
                /// "explicit_bounds" and "bucket_counts" must be omitted and only "count" and
                /// "sum" are known.
                #[derive(Debug, Default, Clone)]
                pub struct HistogramDataPoint {
                    /// The set of key/value pairs that uniquely identify the timeseries from
                    /// where this point belongs. The list may be empty (may contain 0 elements).
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// StartTimeUnixNano is optional but strongly encouraged, see the
                    /// the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#start_time_unix_nano: u64,
                    /// TimeUnixNano is required, see the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#time_unix_nano: u64,
                    /// count is the number of values in the population. Must be non-negative. This
                    /// value must be equal to the sum of the "count" fields in buckets if a
                    /// histogram is provided.
                    pub r#count: u64,
                    /// sum of the values in the population. If count is zero then this field
                    /// must be zero.
                    ///
                    /// Note: Sum should only be filled out when measuring non-negative discrete
                    /// events, and is assumed to be monotonic over the values of these events.
                    /// Negative events *can* be recorded, but sum should not be filled out when
                    /// doing so.  This is specifically to enforce compatibility w/ OpenMetrics,
                    /// see: https://github.com/prometheus/OpenMetrics/blob/v1.0.0/specification/OpenMetrics.md#histogram
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#sum: f64,
                    /// bucket_counts is an optional field contains the count values of histogram
                    /// for each bucket.
                    ///
                    /// The sum of the bucket_counts must equal the value in the count field.
                    ///
                    /// The number of elements in bucket_counts array must be by one greater than
                    /// the number of elements in explicit_bounds array. The exception to this rule
                    /// is when the length of bucket_counts is 0, then the length of explicit_bounds
                    /// must also be 0.
                    pub r#bucket_counts: ::std::vec::Vec<u64>,
                    /// explicit_bounds specifies buckets with explicitly defined bounds for values.
                    ///
                    /// The boundaries for bucket at index i are:
                    ///
                    /// (-infinity, explicit_bounds[i]] for i == 0
                    /// (explicit_bounds[i-1], explicit_bounds[i]] for 0 < i < size(explicit_bounds)
                    /// (explicit_bounds[i-1], +infinity) for i == size(explicit_bounds)
                    ///
                    /// The values in the explicit_bounds array must be strictly increasing.
                    ///
                    /// Histogram buckets are inclusive of their upper boundary, except the last
                    /// bucket where the boundary is at infinity. This format is intentionally
                    /// compatible with the OpenMetrics histogram definition.
                    ///
                    /// If bucket_counts length is 0 then explicit_bounds length must also be 0,
                    /// otherwise the data point is invalid.
                    pub r#explicit_bounds: ::std::vec::Vec<f64>,
                    /// (Optional) List of exemplars collected from
                    /// measurements that were used to form the data point
                    pub r#exemplars: ::std::vec::Vec<Exemplar>,
                    /// Flags that apply to this specific data point.  See DataPointFlags
                    /// for the available flags and their meaning.
                    pub r#flags: u32,
                    /// min is the minimum value over (start_time, end_time].
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#min: f64,
                    /// max is the maximum value over (start_time, end_time].
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#max: f64,
                    /// Tracks presence of optional and message fields
                    pub _has: HistogramDataPoint_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for HistogramDataPoint {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#attributes == other.r#attributes);
                        ret
                            &= (self.r#start_time_unix_nano
                                == other.r#start_time_unix_nano);
                        ret &= (self.r#time_unix_nano == other.r#time_unix_nano);
                        ret &= (self.r#count == other.r#count);
                        ret &= (self.r#sum() == other.r#sum());
                        ret &= (self.r#bucket_counts == other.r#bucket_counts);
                        ret &= (self.r#explicit_bounds == other.r#explicit_bounds);
                        ret &= (self.r#exemplars == other.r#exemplars);
                        ret &= (self.r#flags == other.r#flags);
                        ret &= (self.r#min() == other.r#min());
                        ret &= (self.r#max() == other.r#max());
                        ret
                    }
                }
                impl HistogramDataPoint {
                    /// Return a reference to `start_time_unix_nano`
                    #[inline]
                    pub fn r#start_time_unix_nano(&self) -> &u64 {
                        &self.r#start_time_unix_nano
                    }
                    /// Return a mutable reference to `start_time_unix_nano`
                    #[inline]
                    pub fn mut_start_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#start_time_unix_nano
                    }
                    /// Set the value of `start_time_unix_nano`
                    #[inline]
                    pub fn set_start_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `start_time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_start_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `time_unix_nano`
                    #[inline]
                    pub fn r#time_unix_nano(&self) -> &u64 {
                        &self.r#time_unix_nano
                    }
                    /// Return a mutable reference to `time_unix_nano`
                    #[inline]
                    pub fn mut_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#time_unix_nano
                    }
                    /// Set the value of `time_unix_nano`
                    #[inline]
                    pub fn set_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `count`
                    #[inline]
                    pub fn r#count(&self) -> &u64 {
                        &self.r#count
                    }
                    /// Return a mutable reference to `count`
                    #[inline]
                    pub fn mut_count(&mut self) -> &mut u64 {
                        &mut self.r#count
                    }
                    /// Set the value of `count`
                    #[inline]
                    pub fn set_count(&mut self, value: u64) -> &mut Self {
                        self.r#count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_count(mut self, value: u64) -> Self {
                        self.r#count = value.into();
                        self
                    }
                    /// Return a reference to `sum` as an `Option`
                    #[inline]
                    pub fn r#sum(&self) -> ::core::option::Option<&f64> {
                        self._has.r#sum().then_some(&self.r#sum)
                    }
                    /// Set the value and presence of `sum`
                    #[inline]
                    pub fn set_sum(&mut self, value: f64) -> &mut Self {
                        self._has.set_sum();
                        self.r#sum = value.into();
                        self
                    }
                    /// Return a mutable reference to `sum` as an `Option`
                    #[inline]
                    pub fn mut_sum(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#sum().then_some(&mut self.r#sum)
                    }
                    /// Clear the presence of `sum`
                    #[inline]
                    pub fn clear_sum(&mut self) -> &mut Self {
                        self._has.clear_sum();
                        self
                    }
                    /// Take the value of `sum` and clear its presence
                    #[inline]
                    pub fn take_sum(&mut self) -> ::core::option::Option<f64> {
                        let val = self
                            ._has
                            .r#sum()
                            .then(|| ::core::mem::take(&mut self.r#sum));
                        self._has.clear_sum();
                        val
                    }
                    /// Builder method that sets the value of `sum`. Useful for initializing the message.
                    #[inline]
                    pub fn init_sum(mut self, value: f64) -> Self {
                        self.set_sum(value);
                        self
                    }
                    /// Return a reference to `flags`
                    #[inline]
                    pub fn r#flags(&self) -> &u32 {
                        &self.r#flags
                    }
                    /// Return a mutable reference to `flags`
                    #[inline]
                    pub fn mut_flags(&mut self) -> &mut u32 {
                        &mut self.r#flags
                    }
                    /// Set the value of `flags`
                    #[inline]
                    pub fn set_flags(&mut self, value: u32) -> &mut Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Builder method that sets the value of `flags`. Useful for initializing the message.
                    #[inline]
                    pub fn init_flags(mut self, value: u32) -> Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Return a reference to `min` as an `Option`
                    #[inline]
                    pub fn r#min(&self) -> ::core::option::Option<&f64> {
                        self._has.r#min().then_some(&self.r#min)
                    }
                    /// Set the value and presence of `min`
                    #[inline]
                    pub fn set_min(&mut self, value: f64) -> &mut Self {
                        self._has.set_min();
                        self.r#min = value.into();
                        self
                    }
                    /// Return a mutable reference to `min` as an `Option`
                    #[inline]
                    pub fn mut_min(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#min().then_some(&mut self.r#min)
                    }
                    /// Clear the presence of `min`
                    #[inline]
                    pub fn clear_min(&mut self) -> &mut Self {
                        self._has.clear_min();
                        self
                    }
                    /// Take the value of `min` and clear its presence
                    #[inline]
                    pub fn take_min(&mut self) -> ::core::option::Option<f64> {
                        let val = self
                            ._has
                            .r#min()
                            .then(|| ::core::mem::take(&mut self.r#min));
                        self._has.clear_min();
                        val
                    }
                    /// Builder method that sets the value of `min`. Useful for initializing the message.
                    #[inline]
                    pub fn init_min(mut self, value: f64) -> Self {
                        self.set_min(value);
                        self
                    }
                    /// Return a reference to `max` as an `Option`
                    #[inline]
                    pub fn r#max(&self) -> ::core::option::Option<&f64> {
                        self._has.r#max().then_some(&self.r#max)
                    }
                    /// Set the value and presence of `max`
                    #[inline]
                    pub fn set_max(&mut self, value: f64) -> &mut Self {
                        self._has.set_max();
                        self.r#max = value.into();
                        self
                    }
                    /// Return a mutable reference to `max` as an `Option`
                    #[inline]
                    pub fn mut_max(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#max().then_some(&mut self.r#max)
                    }
                    /// Clear the presence of `max`
                    #[inline]
                    pub fn clear_max(&mut self) -> &mut Self {
                        self._has.clear_max();
                        self
                    }
                    /// Take the value of `max` and clear its presence
                    #[inline]
                    pub fn take_max(&mut self) -> ::core::option::Option<f64> {
                        let val = self
                            ._has
                            .r#max()
                            .then(|| ::core::mem::take(&mut self.r#max));
                        self._has.clear_max();
                        val
                    }
                    /// Builder method that sets the value of `max`. Useful for initializing the message.
                    #[inline]
                    pub fn init_max(mut self, value: f64) -> Self {
                        self.set_max(value);
                        self
                    }
                }
                impl ::micropb::MessageDecode for HistogramDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.HistogramDataPoint.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.HistogramDataPoint.bucket_counts) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.HistogramDataPoint.explicit_bounds) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.HistogramDataPoint.exemplars) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self.r#sum() {
                                encoder.encode_varint32(41u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#bucket_counts.iter().enumerate() {
                                encoder.encode_varint32(49u32)?;
                                encoder.encode_fixed64(*val_ref as _)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#explicit_bounds.iter().enumerate()
                            {
                                encoder.encode_varint32(57u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#exemplars.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self.r#min() {
                                encoder.encode_varint32(89u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#max() {
                                encoder.encode_varint32(97u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self.r#sum() {
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
                            for (i, val_ref) in self.r#exemplars.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self.r#min() {
                                size += 1usize + 8;
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#max() {
                                size += 1usize + 8;
                            }
                        }
                        size
                    }
                }
                /// Inner types for `HistogramDataPoint`
                pub mod HistogramDataPoint_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `sum`
                        #[inline]
                        pub const fn r#sum(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `sum`
                        #[inline]
                        pub const fn set_sum(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `sum`
                        #[inline]
                        pub const fn clear_sum(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `sum`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_sum(mut self) -> Self {
                            self.set_sum();
                            self
                        }
                        /// Query presence of `min`
                        #[inline]
                        pub const fn r#min(&self) -> bool {
                            (self.0[0] & 2) != 0
                        }
                        /// Set presence of `min`
                        #[inline]
                        pub const fn set_min(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 2;
                            self
                        }
                        /// Clear presence of `min`
                        #[inline]
                        pub const fn clear_min(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !2;
                            self
                        }
                        /// Builder method that sets the presence of `min`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_min(mut self) -> Self {
                            self.set_min();
                            self
                        }
                        /// Query presence of `max`
                        #[inline]
                        pub const fn r#max(&self) -> bool {
                            (self.0[0] & 4) != 0
                        }
                        /// Set presence of `max`
                        #[inline]
                        pub const fn set_max(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 4;
                            self
                        }
                        /// Clear presence of `max`
                        #[inline]
                        pub const fn clear_max(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !4;
                            self
                        }
                        /// Builder method that sets the presence of `max`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_max(mut self) -> Self {
                            self.set_max();
                            self
                        }
                    }
                }
                /// ExponentialHistogramDataPoint is a single data point in a timeseries that describes the
                /// time-varying values of a ExponentialHistogram of double values. A ExponentialHistogram contains
                /// summary statistics for a population of values, it may optionally contain the
                /// distribution of those values across a set of buckets.
                ///
                #[derive(Debug, Default, Clone)]
                pub struct ExponentialHistogramDataPoint {
                    /// The set of key/value pairs that uniquely identify the timeseries from
                    /// where this point belongs. The list may be empty (may contain 0 elements).
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// StartTimeUnixNano is optional but strongly encouraged, see the
                    /// the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#start_time_unix_nano: u64,
                    /// TimeUnixNano is required, see the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#time_unix_nano: u64,
                    /// The number of values in the population. Must be
                    /// non-negative. This value must be equal to the sum of the "bucket_counts"
                    /// values in the positive and negative Buckets plus the "zero_count" field.
                    pub r#count: u64,
                    /// The sum of the values in the population. If count is zero then this field
                    /// must be zero.
                    ///
                    /// Note: Sum should only be filled out when measuring non-negative discrete
                    /// events, and is assumed to be monotonic over the values of these events.
                    /// Negative events *can* be recorded, but sum should not be filled out when
                    /// doing so.  This is specifically to enforce compatibility w/ OpenMetrics,
                    /// see: https://github.com/prometheus/OpenMetrics/blob/v1.0.0/specification/OpenMetrics.md#histogram
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#sum: f64,
                    /// scale describes the resolution of the histogram.  Boundaries are
                    /// located at powers of the base, where:
                    ///
                    ///   base = (2^(2^-scale))
                    ///
                    /// The histogram bucket identified by `index`, a signed integer,
                    /// contains values that are greater than (base^index) and
                    /// less than or equal to (base^(index+1)).
                    ///
                    /// The positive and negative ranges of the histogram are expressed
                    /// separately.  Negative values are mapped by their absolute value
                    /// into the negative range using the same scale as the positive range.
                    ///
                    /// scale is not restricted by the protocol, as the permissible
                    /// values depend on the range of the data.
                    pub r#scale: i32,
                    /// The count of values that are either exactly zero or
                    /// within the region considered zero by the instrumentation at the
                    /// tolerated degree of precision.  This bucket stores values that
                    /// cannot be expressed using the standard exponential formula as
                    /// well as values that have been rounded to zero.
                    ///
                    /// Implementations MAY consider the zero bucket to have probability
                    /// mass equal to (zero_count / count).
                    pub r#zero_count: u64,
                    /// positive carries the positive range of exponential bucket counts.
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#positive: ExponentialHistogramDataPoint_::Buckets,
                    /// negative carries the negative range of exponential bucket counts.
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#negative: ExponentialHistogramDataPoint_::Buckets,
                    /// Flags that apply to this specific data point.  See DataPointFlags
                    /// for the available flags and their meaning.
                    pub r#flags: u32,
                    /// (Optional) List of exemplars collected from
                    /// measurements that were used to form the data point
                    pub r#exemplars: ::std::vec::Vec<Exemplar>,
                    /// The minimum value over (start_time, end_time].
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#min: f64,
                    /// The maximum value over (start_time, end_time].
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#max: f64,
                    /// ZeroThreshold may be optionally set to convey the width of the zero
                    /// region. Where the zero region is defined as the closed interval
                    /// [-ZeroThreshold, ZeroThreshold].
                    /// When ZeroThreshold is 0, zero count bucket stores values that cannot be
                    /// expressed using the standard exponential formula as well as values that
                    /// have been rounded to zero.
                    pub r#zero_threshold: f64,
                    /// Tracks presence of optional and message fields
                    pub _has: ExponentialHistogramDataPoint_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for ExponentialHistogramDataPoint {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#attributes == other.r#attributes);
                        ret
                            &= (self.r#start_time_unix_nano
                                == other.r#start_time_unix_nano);
                        ret &= (self.r#time_unix_nano == other.r#time_unix_nano);
                        ret &= (self.r#count == other.r#count);
                        ret &= (self.r#sum() == other.r#sum());
                        ret &= (self.r#scale == other.r#scale);
                        ret &= (self.r#zero_count == other.r#zero_count);
                        ret &= (self.r#positive() == other.r#positive());
                        ret &= (self.r#negative() == other.r#negative());
                        ret &= (self.r#flags == other.r#flags);
                        ret &= (self.r#exemplars == other.r#exemplars);
                        ret &= (self.r#min() == other.r#min());
                        ret &= (self.r#max() == other.r#max());
                        ret &= (self.r#zero_threshold == other.r#zero_threshold);
                        ret
                    }
                }
                impl ExponentialHistogramDataPoint {
                    /// Return a reference to `start_time_unix_nano`
                    #[inline]
                    pub fn r#start_time_unix_nano(&self) -> &u64 {
                        &self.r#start_time_unix_nano
                    }
                    /// Return a mutable reference to `start_time_unix_nano`
                    #[inline]
                    pub fn mut_start_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#start_time_unix_nano
                    }
                    /// Set the value of `start_time_unix_nano`
                    #[inline]
                    pub fn set_start_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `start_time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_start_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `time_unix_nano`
                    #[inline]
                    pub fn r#time_unix_nano(&self) -> &u64 {
                        &self.r#time_unix_nano
                    }
                    /// Return a mutable reference to `time_unix_nano`
                    #[inline]
                    pub fn mut_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#time_unix_nano
                    }
                    /// Set the value of `time_unix_nano`
                    #[inline]
                    pub fn set_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `count`
                    #[inline]
                    pub fn r#count(&self) -> &u64 {
                        &self.r#count
                    }
                    /// Return a mutable reference to `count`
                    #[inline]
                    pub fn mut_count(&mut self) -> &mut u64 {
                        &mut self.r#count
                    }
                    /// Set the value of `count`
                    #[inline]
                    pub fn set_count(&mut self, value: u64) -> &mut Self {
                        self.r#count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_count(mut self, value: u64) -> Self {
                        self.r#count = value.into();
                        self
                    }
                    /// Return a reference to `sum` as an `Option`
                    #[inline]
                    pub fn r#sum(&self) -> ::core::option::Option<&f64> {
                        self._has.r#sum().then_some(&self.r#sum)
                    }
                    /// Set the value and presence of `sum`
                    #[inline]
                    pub fn set_sum(&mut self, value: f64) -> &mut Self {
                        self._has.set_sum();
                        self.r#sum = value.into();
                        self
                    }
                    /// Return a mutable reference to `sum` as an `Option`
                    #[inline]
                    pub fn mut_sum(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#sum().then_some(&mut self.r#sum)
                    }
                    /// Clear the presence of `sum`
                    #[inline]
                    pub fn clear_sum(&mut self) -> &mut Self {
                        self._has.clear_sum();
                        self
                    }
                    /// Take the value of `sum` and clear its presence
                    #[inline]
                    pub fn take_sum(&mut self) -> ::core::option::Option<f64> {
                        let val = self
                            ._has
                            .r#sum()
                            .then(|| ::core::mem::take(&mut self.r#sum));
                        self._has.clear_sum();
                        val
                    }
                    /// Builder method that sets the value of `sum`. Useful for initializing the message.
                    #[inline]
                    pub fn init_sum(mut self, value: f64) -> Self {
                        self.set_sum(value);
                        self
                    }
                    /// Return a reference to `scale`
                    #[inline]
                    pub fn r#scale(&self) -> &i32 {
                        &self.r#scale
                    }
                    /// Return a mutable reference to `scale`
                    #[inline]
                    pub fn mut_scale(&mut self) -> &mut i32 {
                        &mut self.r#scale
                    }
                    /// Set the value of `scale`
                    #[inline]
                    pub fn set_scale(&mut self, value: i32) -> &mut Self {
                        self.r#scale = value.into();
                        self
                    }
                    /// Builder method that sets the value of `scale`. Useful for initializing the message.
                    #[inline]
                    pub fn init_scale(mut self, value: i32) -> Self {
                        self.r#scale = value.into();
                        self
                    }
                    /// Return a reference to `zero_count`
                    #[inline]
                    pub fn r#zero_count(&self) -> &u64 {
                        &self.r#zero_count
                    }
                    /// Return a mutable reference to `zero_count`
                    #[inline]
                    pub fn mut_zero_count(&mut self) -> &mut u64 {
                        &mut self.r#zero_count
                    }
                    /// Set the value of `zero_count`
                    #[inline]
                    pub fn set_zero_count(&mut self, value: u64) -> &mut Self {
                        self.r#zero_count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `zero_count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_zero_count(mut self, value: u64) -> Self {
                        self.r#zero_count = value.into();
                        self
                    }
                    /// Return a reference to `positive` as an `Option`
                    #[inline]
                    pub fn r#positive(
                        &self,
                    ) -> ::core::option::Option<
                        &ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#positive().then_some(&self.r#positive)
                    }
                    /// Set the value and presence of `positive`
                    #[inline]
                    pub fn set_positive(
                        &mut self,
                        value: ExponentialHistogramDataPoint_::Buckets,
                    ) -> &mut Self {
                        self._has.set_positive();
                        self.r#positive = value.into();
                        self
                    }
                    /// Return a mutable reference to `positive` as an `Option`
                    #[inline]
                    pub fn mut_positive(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#positive().then_some(&mut self.r#positive)
                    }
                    /// Clear the presence of `positive`
                    #[inline]
                    pub fn clear_positive(&mut self) -> &mut Self {
                        self._has.clear_positive();
                        self
                    }
                    /// Take the value of `positive` and clear its presence
                    #[inline]
                    pub fn take_positive(
                        &mut self,
                    ) -> ::core::option::Option<
                        ExponentialHistogramDataPoint_::Buckets,
                    > {
                        let val = self
                            ._has
                            .r#positive()
                            .then(|| ::core::mem::take(&mut self.r#positive));
                        self._has.clear_positive();
                        val
                    }
                    /// Builder method that sets the value of `positive`. Useful for initializing the message.
                    #[inline]
                    pub fn init_positive(
                        mut self,
                        value: ExponentialHistogramDataPoint_::Buckets,
                    ) -> Self {
                        self.set_positive(value);
                        self
                    }
                    /// Return a reference to `negative` as an `Option`
                    #[inline]
                    pub fn r#negative(
                        &self,
                    ) -> ::core::option::Option<
                        &ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#negative().then_some(&self.r#negative)
                    }
                    /// Set the value and presence of `negative`
                    #[inline]
                    pub fn set_negative(
                        &mut self,
                        value: ExponentialHistogramDataPoint_::Buckets,
                    ) -> &mut Self {
                        self._has.set_negative();
                        self.r#negative = value.into();
                        self
                    }
                    /// Return a mutable reference to `negative` as an `Option`
                    #[inline]
                    pub fn mut_negative(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut ExponentialHistogramDataPoint_::Buckets,
                    > {
                        self._has.r#negative().then_some(&mut self.r#negative)
                    }
                    /// Clear the presence of `negative`
                    #[inline]
                    pub fn clear_negative(&mut self) -> &mut Self {
                        self._has.clear_negative();
                        self
                    }
                    /// Take the value of `negative` and clear its presence
                    #[inline]
                    pub fn take_negative(
                        &mut self,
                    ) -> ::core::option::Option<
                        ExponentialHistogramDataPoint_::Buckets,
                    > {
                        let val = self
                            ._has
                            .r#negative()
                            .then(|| ::core::mem::take(&mut self.r#negative));
                        self._has.clear_negative();
                        val
                    }
                    /// Builder method that sets the value of `negative`. Useful for initializing the message.
                    #[inline]
                    pub fn init_negative(
                        mut self,
                        value: ExponentialHistogramDataPoint_::Buckets,
                    ) -> Self {
                        self.set_negative(value);
                        self
                    }
                    /// Return a reference to `flags`
                    #[inline]
                    pub fn r#flags(&self) -> &u32 {
                        &self.r#flags
                    }
                    /// Return a mutable reference to `flags`
                    #[inline]
                    pub fn mut_flags(&mut self) -> &mut u32 {
                        &mut self.r#flags
                    }
                    /// Set the value of `flags`
                    #[inline]
                    pub fn set_flags(&mut self, value: u32) -> &mut Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Builder method that sets the value of `flags`. Useful for initializing the message.
                    #[inline]
                    pub fn init_flags(mut self, value: u32) -> Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Return a reference to `min` as an `Option`
                    #[inline]
                    pub fn r#min(&self) -> ::core::option::Option<&f64> {
                        self._has.r#min().then_some(&self.r#min)
                    }
                    /// Set the value and presence of `min`
                    #[inline]
                    pub fn set_min(&mut self, value: f64) -> &mut Self {
                        self._has.set_min();
                        self.r#min = value.into();
                        self
                    }
                    /// Return a mutable reference to `min` as an `Option`
                    #[inline]
                    pub fn mut_min(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#min().then_some(&mut self.r#min)
                    }
                    /// Clear the presence of `min`
                    #[inline]
                    pub fn clear_min(&mut self) -> &mut Self {
                        self._has.clear_min();
                        self
                    }
                    /// Take the value of `min` and clear its presence
                    #[inline]
                    pub fn take_min(&mut self) -> ::core::option::Option<f64> {
                        let val = self
                            ._has
                            .r#min()
                            .then(|| ::core::mem::take(&mut self.r#min));
                        self._has.clear_min();
                        val
                    }
                    /// Builder method that sets the value of `min`. Useful for initializing the message.
                    #[inline]
                    pub fn init_min(mut self, value: f64) -> Self {
                        self.set_min(value);
                        self
                    }
                    /// Return a reference to `max` as an `Option`
                    #[inline]
                    pub fn r#max(&self) -> ::core::option::Option<&f64> {
                        self._has.r#max().then_some(&self.r#max)
                    }
                    /// Set the value and presence of `max`
                    #[inline]
                    pub fn set_max(&mut self, value: f64) -> &mut Self {
                        self._has.set_max();
                        self.r#max = value.into();
                        self
                    }
                    /// Return a mutable reference to `max` as an `Option`
                    #[inline]
                    pub fn mut_max(&mut self) -> ::core::option::Option<&mut f64> {
                        self._has.r#max().then_some(&mut self.r#max)
                    }
                    /// Clear the presence of `max`
                    #[inline]
                    pub fn clear_max(&mut self) -> &mut Self {
                        self._has.clear_max();
                        self
                    }
                    /// Take the value of `max` and clear its presence
                    #[inline]
                    pub fn take_max(&mut self) -> ::core::option::Option<f64> {
                        let val = self
                            ._has
                            .r#max()
                            .then(|| ::core::mem::take(&mut self.r#max));
                        self._has.clear_max();
                        val
                    }
                    /// Builder method that sets the value of `max`. Useful for initializing the message.
                    #[inline]
                    pub fn init_max(mut self, value: f64) -> Self {
                        self.set_max(value);
                        self
                    }
                    /// Return a reference to `zero_threshold`
                    #[inline]
                    pub fn r#zero_threshold(&self) -> &f64 {
                        &self.r#zero_threshold
                    }
                    /// Return a mutable reference to `zero_threshold`
                    #[inline]
                    pub fn mut_zero_threshold(&mut self) -> &mut f64 {
                        &mut self.r#zero_threshold
                    }
                    /// Set the value of `zero_threshold`
                    #[inline]
                    pub fn set_zero_threshold(&mut self, value: f64) -> &mut Self {
                        self.r#zero_threshold = value.into();
                        self
                    }
                    /// Builder method that sets the value of `zero_threshold`. Useful for initializing the message.
                    #[inline]
                    pub fn init_zero_threshold(mut self, value: f64) -> Self {
                        self.r#zero_threshold = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ExponentialHistogramDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.ExponentialHistogramDataPoint.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            ExponentialHistogramDataPoint_::Buckets as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            ExponentialHistogramDataPoint_::Buckets as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.ExponentialHistogramDataPoint.exemplars) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self.r#sum() {
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
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#positive()
                            {
                                encoder.encode_varint32(66u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#negative()
                            {
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
                            for (i, val_ref) in self.r#exemplars.iter().enumerate() {
                                encoder.encode_varint32(90u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#min() {
                                encoder.encode_varint32(97u32)?;
                                encoder.encode_double(*val_ref)?;
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#max() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self.r#sum() {
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
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#positive()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#negative()
                            {
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
                            for (i, val_ref) in self.r#exemplars.iter().enumerate() {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#min() {
                                size += 1usize + 8;
                            }
                        }
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#max() {
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
                /// Inner types for `ExponentialHistogramDataPoint`
                pub mod ExponentialHistogramDataPoint_ {
                    /// Buckets are a set of bucket counts, encoded in a contiguous array
                    /// of counts.
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct Buckets {
                        /// The bucket index of the first entry in the bucket_counts array.
                        ///
                        /// Note: This uses a varint encoding as a simple form of compression.
                        pub r#offset: i32,
                        /// An array of count values, where bucket_counts[i] carries
                        /// the count of the bucket at index (offset+i). bucket_counts[i] is the count
                        /// of values greater than base^(offset+i) and less than or equal to
                        /// base^(offset+i+1).
                        ///
                        /// Note: By contrast, the explicit HistogramDataPoint uses
                        /// fixed64.  This field is expected to have many buckets,
                        /// especially zeros, so uint64 has been selected to ensure
                        /// varint encoding.
                        pub r#bucket_counts: ::std::vec::Vec<u64>,
                    }
                    impl Buckets {
                        /// Return a reference to `offset`
                        #[inline]
                        pub fn r#offset(&self) -> &i32 {
                            &self.r#offset
                        }
                        /// Return a mutable reference to `offset`
                        #[inline]
                        pub fn mut_offset(&mut self) -> &mut i32 {
                            &mut self.r#offset
                        }
                        /// Set the value of `offset`
                        #[inline]
                        pub fn set_offset(&mut self, value: i32) -> &mut Self {
                            self.r#offset = value.into();
                            self
                        }
                        /// Builder method that sets the value of `offset`. Useful for initializing the message.
                        #[inline]
                        pub fn init_offset(mut self, value: i32) -> Self {
                            self.r#offset = value.into();
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for Buckets {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
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
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(5usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::core::result::Result::<
                                usize,
                                &'static str,
                            >::Err(
                                "(.opentelemetry.proto.metrics.v1.Buckets.bucket_counts) unbounded vec",
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
                            {
                                let val_ref = &self.r#offset;
                                if *val_ref != 0 {
                                    encoder.encode_varint32(8u32)?;
                                    encoder.encode_sint32(*val_ref as _)?;
                                }
                            }
                            {
                                for (i, val_ref) in self.r#bucket_counts.iter().enumerate()
                                {
                                    encoder.encode_varint32(16u32)?;
                                    encoder.encode_varint64(*val_ref as _)?;
                                }
                            }
                            Ok(())
                        }
                        fn compute_size(&self) -> usize {
                            use ::micropb::{PbMap, FieldEncode};
                            let mut size = 0;
                            {
                                let val_ref = &self.r#offset;
                                if *val_ref != 0 {
                                    size
                                        += 1usize + ::micropb::size::sizeof_sint32(*val_ref as _);
                                }
                            }
                            {
                                for (i, val_ref) in self.r#bucket_counts.iter().enumerate()
                                {
                                    size
                                        += 1usize + ::micropb::size::sizeof_varint64(*val_ref as _);
                                }
                            }
                            size
                        }
                    }
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `sum`
                        #[inline]
                        pub const fn r#sum(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `sum`
                        #[inline]
                        pub const fn set_sum(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `sum`
                        #[inline]
                        pub const fn clear_sum(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `sum`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_sum(mut self) -> Self {
                            self.set_sum();
                            self
                        }
                        /// Query presence of `positive`
                        #[inline]
                        pub const fn r#positive(&self) -> bool {
                            (self.0[0] & 2) != 0
                        }
                        /// Set presence of `positive`
                        #[inline]
                        pub const fn set_positive(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 2;
                            self
                        }
                        /// Clear presence of `positive`
                        #[inline]
                        pub const fn clear_positive(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !2;
                            self
                        }
                        /// Builder method that sets the presence of `positive`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_positive(mut self) -> Self {
                            self.set_positive();
                            self
                        }
                        /// Query presence of `negative`
                        #[inline]
                        pub const fn r#negative(&self) -> bool {
                            (self.0[0] & 4) != 0
                        }
                        /// Set presence of `negative`
                        #[inline]
                        pub const fn set_negative(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 4;
                            self
                        }
                        /// Clear presence of `negative`
                        #[inline]
                        pub const fn clear_negative(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !4;
                            self
                        }
                        /// Builder method that sets the presence of `negative`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_negative(mut self) -> Self {
                            self.set_negative();
                            self
                        }
                        /// Query presence of `min`
                        #[inline]
                        pub const fn r#min(&self) -> bool {
                            (self.0[0] & 8) != 0
                        }
                        /// Set presence of `min`
                        #[inline]
                        pub const fn set_min(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 8;
                            self
                        }
                        /// Clear presence of `min`
                        #[inline]
                        pub const fn clear_min(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !8;
                            self
                        }
                        /// Builder method that sets the presence of `min`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_min(mut self) -> Self {
                            self.set_min();
                            self
                        }
                        /// Query presence of `max`
                        #[inline]
                        pub const fn r#max(&self) -> bool {
                            (self.0[0] & 16) != 0
                        }
                        /// Set presence of `max`
                        #[inline]
                        pub const fn set_max(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 16;
                            self
                        }
                        /// Clear presence of `max`
                        #[inline]
                        pub const fn clear_max(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !16;
                            self
                        }
                        /// Builder method that sets the presence of `max`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_max(mut self) -> Self {
                            self.set_max();
                            self
                        }
                    }
                }
                /// SummaryDataPoint is a single data point in a timeseries that describes the
                /// time-varying values of a Summary metric. The count and sum fields represent
                /// cumulative values.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct SummaryDataPoint {
                    /// The set of key/value pairs that uniquely identify the timeseries from
                    /// where this point belongs. The list may be empty (may contain 0 elements).
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// StartTimeUnixNano is optional but strongly encouraged, see the
                    /// the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#start_time_unix_nano: u64,
                    /// TimeUnixNano is required, see the detailed comments above Metric.
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#time_unix_nano: u64,
                    /// count is the number of values in the population. Must be non-negative.
                    pub r#count: u64,
                    /// sum of the values in the population. If count is zero then this field
                    /// must be zero.
                    ///
                    /// Note: Sum should only be filled out when measuring non-negative discrete
                    /// events, and is assumed to be monotonic over the values of these events.
                    /// Negative events *can* be recorded, but sum should not be filled out when
                    /// doing so.  This is specifically to enforce compatibility w/ OpenMetrics,
                    /// see: https://github.com/prometheus/OpenMetrics/blob/v1.0.0/specification/OpenMetrics.md#summary
                    pub r#sum: f64,
                    /// (Optional) list of values at different quantiles of the distribution calculated
                    /// from the current snapshot. The quantiles must be strictly increasing.
                    pub r#quantile_values: ::std::vec::Vec<
                        SummaryDataPoint_::ValueAtQuantile,
                    >,
                    /// Flags that apply to this specific data point.  See DataPointFlags
                    /// for the available flags and their meaning.
                    pub r#flags: u32,
                }
                impl SummaryDataPoint {
                    /// Return a reference to `start_time_unix_nano`
                    #[inline]
                    pub fn r#start_time_unix_nano(&self) -> &u64 {
                        &self.r#start_time_unix_nano
                    }
                    /// Return a mutable reference to `start_time_unix_nano`
                    #[inline]
                    pub fn mut_start_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#start_time_unix_nano
                    }
                    /// Set the value of `start_time_unix_nano`
                    #[inline]
                    pub fn set_start_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `start_time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_start_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `time_unix_nano`
                    #[inline]
                    pub fn r#time_unix_nano(&self) -> &u64 {
                        &self.r#time_unix_nano
                    }
                    /// Return a mutable reference to `time_unix_nano`
                    #[inline]
                    pub fn mut_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#time_unix_nano
                    }
                    /// Set the value of `time_unix_nano`
                    #[inline]
                    pub fn set_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `count`
                    #[inline]
                    pub fn r#count(&self) -> &u64 {
                        &self.r#count
                    }
                    /// Return a mutable reference to `count`
                    #[inline]
                    pub fn mut_count(&mut self) -> &mut u64 {
                        &mut self.r#count
                    }
                    /// Set the value of `count`
                    #[inline]
                    pub fn set_count(&mut self, value: u64) -> &mut Self {
                        self.r#count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_count(mut self, value: u64) -> Self {
                        self.r#count = value.into();
                        self
                    }
                    /// Return a reference to `sum`
                    #[inline]
                    pub fn r#sum(&self) -> &f64 {
                        &self.r#sum
                    }
                    /// Return a mutable reference to `sum`
                    #[inline]
                    pub fn mut_sum(&mut self) -> &mut f64 {
                        &mut self.r#sum
                    }
                    /// Set the value of `sum`
                    #[inline]
                    pub fn set_sum(&mut self, value: f64) -> &mut Self {
                        self.r#sum = value.into();
                        self
                    }
                    /// Builder method that sets the value of `sum`. Useful for initializing the message.
                    #[inline]
                    pub fn init_sum(mut self, value: f64) -> Self {
                        self.r#sum = value.into();
                        self
                    }
                    /// Return a reference to `flags`
                    #[inline]
                    pub fn r#flags(&self) -> &u32 {
                        &self.r#flags
                    }
                    /// Return a mutable reference to `flags`
                    #[inline]
                    pub fn mut_flags(&mut self) -> &mut u32 {
                        &mut self.r#flags
                    }
                    /// Set the value of `flags`
                    #[inline]
                    pub fn set_flags(&mut self, value: u32) -> &mut Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Builder method that sets the value of `flags`. Useful for initializing the message.
                    #[inline]
                    pub fn init_flags(mut self, value: u32) -> Self {
                        self.r#flags = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for SummaryDataPoint {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.SummaryDataPoint.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.SummaryDataPoint.quantile_values) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            for (i, val_ref) in self.r#quantile_values.iter().enumerate()
                            {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            for (i, val_ref) in self.r#quantile_values.iter().enumerate()
                            {
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
                /// Inner types for `SummaryDataPoint`
                pub mod SummaryDataPoint_ {
                    /// Represents the value at a given quantile of a distribution.
                    ///
                    /// To record Min and Max values following conventions are used:
                    /// - The 1.0 quantile is equivalent to the maximum value observed.
                    /// - The 0.0 quantile is equivalent to the minimum value observed.
                    ///
                    /// See the following issue for more context:
                    /// https://github.com/open-telemetry/opentelemetry-proto/issues/125
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct ValueAtQuantile {
                        /// The quantile of a distribution. Must be in the interval
                        /// [0.0, 1.0].
                        pub r#quantile: f64,
                        /// The value at the given quantile of a distribution.
                        ///
                        /// Quantile values must NOT be negative.
                        pub r#value: f64,
                    }
                    impl ValueAtQuantile {
                        /// Return a reference to `quantile`
                        #[inline]
                        pub fn r#quantile(&self) -> &f64 {
                            &self.r#quantile
                        }
                        /// Return a mutable reference to `quantile`
                        #[inline]
                        pub fn mut_quantile(&mut self) -> &mut f64 {
                            &mut self.r#quantile
                        }
                        /// Set the value of `quantile`
                        #[inline]
                        pub fn set_quantile(&mut self, value: f64) -> &mut Self {
                            self.r#quantile = value.into();
                            self
                        }
                        /// Builder method that sets the value of `quantile`. Useful for initializing the message.
                        #[inline]
                        pub fn init_quantile(mut self, value: f64) -> Self {
                            self.r#quantile = value.into();
                            self
                        }
                        /// Return a reference to `value`
                        #[inline]
                        pub fn r#value(&self) -> &f64 {
                            &self.r#value
                        }
                        /// Return a mutable reference to `value`
                        #[inline]
                        pub fn mut_value(&mut self) -> &mut f64 {
                            &mut self.r#value
                        }
                        /// Set the value of `value`
                        #[inline]
                        pub fn set_value(&mut self, value: f64) -> &mut Self {
                            self.r#value = value.into();
                            self
                        }
                        /// Builder method that sets the value of `value`. Useful for initializing the message.
                        #[inline]
                        pub fn init_value(mut self, value: f64) -> Self {
                            self.r#value = value.into();
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for ValueAtQuantile {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
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
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
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
                            use ::micropb::{PbMap, FieldEncode};
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
                /// A representation of an exemplar, which is a sample input measurement.
                /// Exemplars also hold information about the environment when the measurement
                /// was recorded, for example the span and trace ID of the active span when the
                /// exemplar was recorded.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Exemplar {
                    /// The set of key/value pairs that were filtered out by the aggregator, but
                    /// recorded alongside the original measurement. Only key/value pairs that were
                    /// filtered out by the aggregator should be included
                    pub r#filtered_attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// time_unix_nano is the exact time when this exemplar was recorded
                    ///
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January
                    /// 1970.
                    pub r#time_unix_nano: u64,
                    /// (Optional) Span ID of the exemplar trace.
                    /// span_id may be missing if the measurement is not recorded inside a trace
                    /// or if the trace is not sampled.
                    pub r#span_id: ::std::vec::Vec<u8>,
                    /// (Optional) Trace ID of the exemplar trace.
                    /// trace_id may be missing if the measurement is not recorded inside a trace
                    /// or if the trace is not sampled.
                    pub r#trace_id: ::std::vec::Vec<u8>,
                    /// The value of the measurement that was recorded. An exemplar is
                    /// considered invalid when one of the recognized value fields is not present
                    /// inside this oneof.
                    pub r#value: ::core::option::Option<Exemplar_::Value>,
                }
                impl Exemplar {
                    /// Return a reference to `time_unix_nano`
                    #[inline]
                    pub fn r#time_unix_nano(&self) -> &u64 {
                        &self.r#time_unix_nano
                    }
                    /// Return a mutable reference to `time_unix_nano`
                    #[inline]
                    pub fn mut_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#time_unix_nano
                    }
                    /// Set the value of `time_unix_nano`
                    #[inline]
                    pub fn set_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `span_id`
                    #[inline]
                    pub fn r#span_id(&self) -> &::std::vec::Vec<u8> {
                        &self.r#span_id
                    }
                    /// Return a mutable reference to `span_id`
                    #[inline]
                    pub fn mut_span_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                        &mut self.r#span_id
                    }
                    /// Set the value of `span_id`
                    #[inline]
                    pub fn set_span_id(
                        &mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> &mut Self {
                        self.r#span_id = value.into();
                        self
                    }
                    /// Builder method that sets the value of `span_id`. Useful for initializing the message.
                    #[inline]
                    pub fn init_span_id(mut self, value: ::std::vec::Vec<u8>) -> Self {
                        self.r#span_id = value.into();
                        self
                    }
                    /// Return a reference to `trace_id`
                    #[inline]
                    pub fn r#trace_id(&self) -> &::std::vec::Vec<u8> {
                        &self.r#trace_id
                    }
                    /// Return a mutable reference to `trace_id`
                    #[inline]
                    pub fn mut_trace_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                        &mut self.r#trace_id
                    }
                    /// Set the value of `trace_id`
                    #[inline]
                    pub fn set_trace_id(
                        &mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> &mut Self {
                        self.r#trace_id = value.into();
                        self
                    }
                    /// Builder method that sets the value of `trace_id`. Useful for initializing the message.
                    #[inline]
                    pub fn init_trace_id(mut self, value: ::std::vec::Vec<u8>) -> Self {
                        self.r#trace_id = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for Exemplar {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.metrics.v1.Exemplar.filtered_attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.metrics.v1.Exemplar.span_id) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.metrics.v1.Exemplar.trace_id) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match 'oneof: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    if size > max_size {
                                        max_size = size;
                                    }
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'oneof (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        } {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self
                                .r#filtered_attributes
                                .iter()
                                .enumerate()
                            {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self
                                .r#filtered_attributes
                                .iter()
                                .enumerate()
                            {
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
                /// Inner types for `Exemplar`
                pub mod Exemplar_ {
                    /// The value of the measurement that was recorded. An exemplar is
                    /// considered invalid when one of the recognized value fields is not present
                    /// inside this oneof.
                    #[derive(Debug, PartialEq, Clone)]
                    pub enum Value {
                        AsDouble(f64),
                        AsInt(i64),
                    }
                }
                /// AggregationTemporality defines how a metric aggregator reports aggregated
                /// values. It describes how those values relate to the time interval over
                /// which they are aggregated.
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct AggregationTemporality(pub i32);
                impl AggregationTemporality {
                    /// Maximum encoded size of the enum
                    pub const _MAX_SIZE: usize = 10usize;
                    /// UNSPECIFIED is the default AggregationTemporality, it MUST not be used.
                    pub const Unspecified: Self = Self(0);
                    /// DELTA is an AggregationTemporality for a metric aggregator which reports
                    /// changes since last report time. Successive metrics contain aggregation of
                    /// values from continuous and non-overlapping intervals.
                    ///
                    /// The values for a DELTA metric are based only on the time interval
                    /// associated with one measurement cycle. There is no dependency on
                    /// previous measurements like is the case for CUMULATIVE metrics.
                    ///
                    /// For example, consider a system measuring the number of requests that
                    /// it receives and reports the sum of these requests every second as a
                    /// DELTA metric:
                    ///
                    ///   1. The system starts receiving at time=t_0.
                    ///   2. A request is received, the system measures 1 request.
                    ///   3. A request is received, the system measures 1 request.
                    ///   4. A request is received, the system measures 1 request.
                    ///   5. The 1 second collection cycle ends. A metric is exported for the
                    ///      number of requests received over the interval of time t_0 to
                    ///      t_0+1 with a value of 3.
                    ///   6. A request is received, the system measures 1 request.
                    ///   7. A request is received, the system measures 1 request.
                    ///   8. The 1 second collection cycle ends. A metric is exported for the
                    ///      number of requests received over the interval of time t_0+1 to
                    ///      t_0+2 with a value of 2.
                    pub const Delta: Self = Self(1);
                    /// CUMULATIVE is an AggregationTemporality for a metric aggregator which
                    /// reports changes since a fixed start time. This means that current values
                    /// of a CUMULATIVE metric depend on all previous measurements since the
                    /// start time. Because of this, the sender is required to retain this state
                    /// in some form. If this state is lost or invalidated, the CUMULATIVE metric
                    /// values MUST be reset and a new fixed start time following the last
                    /// reported measurement time sent MUST be used.
                    ///
                    /// For example, consider a system measuring the number of requests that
                    /// it receives and reports the sum of these requests every second as a
                    /// CUMULATIVE metric:
                    ///
                    ///   1. The system starts receiving at time=t_0.
                    ///   2. A request is received, the system measures 1 request.
                    ///   3. A request is received, the system measures 1 request.
                    ///   4. A request is received, the system measures 1 request.
                    ///   5. The 1 second collection cycle ends. A metric is exported for the
                    ///      number of requests received over the interval of time t_0 to
                    ///      t_0+1 with a value of 3.
                    ///   6. A request is received, the system measures 1 request.
                    ///   7. A request is received, the system measures 1 request.
                    ///   8. The 1 second collection cycle ends. A metric is exported for the
                    ///      number of requests received over the interval of time t_0 to
                    ///      t_0+2 with a value of 5.
                    ///   9. The system experiences a fault and loses state.
                    ///   10. The system recovers and resumes receiving at time=t_1.
                    ///   11. A request is received, the system measures 1 request.
                    ///   12. The 1 second collection cycle ends. A metric is exported for the
                    ///      number of requests received over the interval of time t_1 to
                    ///      t_0+1 with a value of 1.
                    ///
                    /// Note: Even though, when reporting changes since last report time, using
                    /// CUMULATIVE is valid, it is not recommended. This may cause problems for
                    /// systems that do not use start_time to determine when the aggregation
                    /// value was reset (e.g. Prometheus).
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
                /// DataPointFlags is defined as a protobuf 'uint32' type and is to be used as a
                /// bit-field representing 32 distinct boolean flags.  Each flag defined in this
                /// enum is a bit-mask.  To test the presence of a single flag in the flags of
                /// a data point, for example, use an expression like:
                ///
                ///   (point.flags & DATA_POINT_FLAGS_NO_RECORDED_VALUE_MASK) == DATA_POINT_FLAGS_NO_RECORDED_VALUE_MASK
                ///
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct DataPointFlags(pub i32);
                impl DataPointFlags {
                    /// Maximum encoded size of the enum
                    pub const _MAX_SIZE: usize = 10usize;
                    /// The zero value for the enum. Should not be used for comparisons.
                    /// Instead use bitwise "and" with the appropriate mask as shown above.
                    pub const DoNotUse: Self = Self(0);
                    /// This DataPoint is valid but has no recorded value.  This value
                    /// SHOULD be used to reflect explicitly missing data in a series, as
                    /// for an equivalent to the Prometheus "staleness marker".
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
                /// TracesData represents the traces data that can be stored in a persistent storage,
                /// OR can be embedded by other protocols that transfer OTLP traces data but do
                /// not implement the OTLP protocol.
                ///
                /// The main difference between this message and collector protocol is that
                /// in this message there will not be any "control" or "metadata" specific to
                /// OTLP protocol.
                ///
                /// When new fields are added into this message, the OTLP request MUST be updated
                /// as well.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct TracesData {
                    /// An array of ResourceSpans.
                    /// For data coming from a single resource this array will typically contain
                    /// one element. Intermediary nodes that receive data from multiple origins
                    /// typically batch the data before forwarding further and in that case this
                    /// array will contain multiple elements.
                    pub r#resource_spans: ::std::vec::Vec<ResourceSpans>,
                }
                impl TracesData {}
                impl ::micropb::MessageDecode for TracesData {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.trace.v1.TracesData.resource_spans) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            for (i, val_ref) in self.r#resource_spans.iter().enumerate()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            for (i, val_ref) in self.r#resource_spans.iter().enumerate()
                            {
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
                /// A collection of ScopeSpans from a Resource.
                #[derive(Debug, Default, Clone)]
                pub struct ResourceSpans {
                    /// The resource for the spans in this message.
                    /// If this field is not set then no resource info is known.
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#resource: super::super::resource_::v1_::Resource,
                    /// A list of ScopeSpans that originate from a resource.
                    pub r#scope_spans: ::std::vec::Vec<ScopeSpans>,
                    /// The Schema URL, if known. This is the identifier of the Schema that the resource data
                    /// is recorded in. Notably, the last part of the URL path is the version number of the
                    /// schema: http[s]://server[:port]/path/<version>. To learn more about Schema URL see
                    /// https://opentelemetry.io/docs/specs/otel/schemas/#schema-url
                    /// This schema_url applies to the data in the "resource" field. It does not apply
                    /// to the data in the "scope_spans" field which have their own schema_url field.
                    pub r#schema_url: ::std::string::String,
                    /// Tracks presence of optional and message fields
                    pub _has: ResourceSpans_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for ResourceSpans {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#resource() == other.r#resource());
                        ret &= (self.r#scope_spans == other.r#scope_spans);
                        ret &= (self.r#schema_url == other.r#schema_url);
                        ret
                    }
                }
                impl ResourceSpans {
                    /// Return a reference to `resource` as an `Option`
                    #[inline]
                    pub fn r#resource(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&self.r#resource)
                    }
                    /// Set the value and presence of `resource`
                    #[inline]
                    pub fn set_resource(
                        &mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) -> &mut Self {
                        self._has.set_resource();
                        self.r#resource = value.into();
                        self
                    }
                    /// Return a mutable reference to `resource` as an `Option`
                    #[inline]
                    pub fn mut_resource(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::resource_::v1_::Resource,
                    > {
                        self._has.r#resource().then_some(&mut self.r#resource)
                    }
                    /// Clear the presence of `resource`
                    #[inline]
                    pub fn clear_resource(&mut self) -> &mut Self {
                        self._has.clear_resource();
                        self
                    }
                    /// Take the value of `resource` and clear its presence
                    #[inline]
                    pub fn take_resource(
                        &mut self,
                    ) -> ::core::option::Option<super::super::resource_::v1_::Resource> {
                        let val = self
                            ._has
                            .r#resource()
                            .then(|| ::core::mem::take(&mut self.r#resource));
                        self._has.clear_resource();
                        val
                    }
                    /// Builder method that sets the value of `resource`. Useful for initializing the message.
                    #[inline]
                    pub fn init_resource(
                        mut self,
                        value: super::super::resource_::v1_::Resource,
                    ) -> Self {
                        self.set_resource(value);
                        self
                    }
                    /// Return a reference to `schema_url`
                    #[inline]
                    pub fn r#schema_url(&self) -> &::std::string::String {
                        &self.r#schema_url
                    }
                    /// Return a mutable reference to `schema_url`
                    #[inline]
                    pub fn mut_schema_url(&mut self) -> &mut ::std::string::String {
                        &mut self.r#schema_url
                    }
                    /// Set the value of `schema_url`
                    #[inline]
                    pub fn set_schema_url(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Builder method that sets the value of `schema_url`. Useful for initializing the message.
                    #[inline]
                    pub fn init_schema_url(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#schema_url = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ResourceSpans {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            super::super::resource_::v1_::Resource as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.trace.v1.ResourceSpans.scope_spans) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.ResourceSpans.schema_url) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#resource()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#scope_spans.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#resource()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for (i, val_ref) in self.r#scope_spans.iter().enumerate() {
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
                /// Inner types for `ResourceSpans`
                pub mod ResourceSpans_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `resource`
                        #[inline]
                        pub const fn r#resource(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `resource`
                        #[inline]
                        pub const fn set_resource(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `resource`
                        #[inline]
                        pub const fn clear_resource(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `resource`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_resource(mut self) -> Self {
                            self.set_resource();
                            self
                        }
                    }
                }
                /// A collection of Spans produced by an InstrumentationScope.
                #[derive(Debug, Default, Clone)]
                pub struct ScopeSpans {
                    /// The instrumentation scope information for the spans in this message.
                    /// Semantically when InstrumentationScope isn't set, it is equivalent with
                    /// an empty instrumentation scope name (unknown).
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#scope: super::super::common_::v1_::InstrumentationScope,
                    /// A list of Spans that originate from an instrumentation scope.
                    pub r#spans: ::std::vec::Vec<Span>,
                    /// The Schema URL, if known. This is the identifier of the Schema that the span data
                    /// is recorded in. Notably, the last part of the URL path is the version number of the
                    /// schema: http[s]://server[:port]/path/<version>. To learn more about Schema URL see
                    /// https://opentelemetry.io/docs/specs/otel/schemas/#schema-url
                    /// This schema_url applies to the data in the "scope" field and all spans and span
                    /// events in the "spans" field.
                    pub r#schema_url: ::std::string::String,
                    /// Tracks presence of optional and message fields
                    pub _has: ScopeSpans_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for ScopeSpans {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#scope() == other.r#scope());
                        ret &= (self.r#spans == other.r#spans);
                        ret &= (self.r#schema_url == other.r#schema_url);
                        ret
                    }
                }
                impl ScopeSpans {
                    /// Return a reference to `scope` as an `Option`
                    #[inline]
                    pub fn r#scope(
                        &self,
                    ) -> ::core::option::Option<
                        &super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&self.r#scope)
                    }
                    /// Set the value and presence of `scope`
                    #[inline]
                    pub fn set_scope(
                        &mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) -> &mut Self {
                        self._has.set_scope();
                        self.r#scope = value.into();
                        self
                    }
                    /// Return a mutable reference to `scope` as an `Option`
                    #[inline]
                    pub fn mut_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        &mut super::super::common_::v1_::InstrumentationScope,
                    > {
                        self._has.r#scope().then_some(&mut self.r#scope)
                    }
                    /// Clear the presence of `scope`
                    #[inline]
                    pub fn clear_scope(&mut self) -> &mut Self {
                        self._has.clear_scope();
                        self
                    }
                    /// Take the value of `scope` and clear its presence
                    #[inline]
                    pub fn take_scope(
                        &mut self,
                    ) -> ::core::option::Option<
                        super::super::common_::v1_::InstrumentationScope,
                    > {
                        let val = self
                            ._has
                            .r#scope()
                            .then(|| ::core::mem::take(&mut self.r#scope));
                        self._has.clear_scope();
                        val
                    }
                    /// Builder method that sets the value of `scope`. Useful for initializing the message.
                    #[inline]
                    pub fn init_scope(
                        mut self,
                        value: super::super::common_::v1_::InstrumentationScope,
                    ) -> Self {
                        self.set_scope(value);
                        self
                    }
                    /// Return a reference to `schema_url`
                    #[inline]
                    pub fn r#schema_url(&self) -> &::std::string::String {
                        &self.r#schema_url
                    }
                    /// Return a mutable reference to `schema_url`
                    #[inline]
                    pub fn mut_schema_url(&mut self) -> &mut ::std::string::String {
                        &mut self.r#schema_url
                    }
                    /// Set the value of `schema_url`
                    #[inline]
                    pub fn set_schema_url(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#schema_url = value.into();
                        self
                    }
                    /// Builder method that sets the value of `schema_url`. Useful for initializing the message.
                    #[inline]
                    pub fn init_schema_url(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#schema_url = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for ScopeSpans {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::micropb::const_map!(<
                            super::super::common_::v1_::InstrumentationScope as
                            ::micropb::MessageEncode > ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.trace.v1.ScopeSpans.spans) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.ScopeSpans.schema_url) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#scope()
                            {
                                encoder.encode_varint32(10u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        {
                            for (i, val_ref) in self.r#spans.iter().enumerate() {
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
                        use ::micropb::{PbMap, FieldEncode};
                        let mut size = 0;
                        {
                            if let ::core::option::Option::Some(val_ref) = self.r#scope()
                            {
                                size
                                    += 1usize
                                        + ::micropb::size::sizeof_len_record(
                                            val_ref.compute_size(),
                                        );
                            }
                        }
                        {
                            for (i, val_ref) in self.r#spans.iter().enumerate() {
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
                /// Inner types for `ScopeSpans`
                pub mod ScopeSpans_ {
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `scope`
                        #[inline]
                        pub const fn r#scope(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `scope`
                        #[inline]
                        pub const fn set_scope(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `scope`
                        #[inline]
                        pub const fn clear_scope(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `scope`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_scope(mut self) -> Self {
                            self.set_scope();
                            self
                        }
                    }
                }
                /// A Span represents a single operation performed by a single component of the system.
                ///
                /// The next available field id is 17.
                #[derive(Debug, Default, Clone)]
                pub struct Span {
                    /// A unique identifier for a trace. All spans from the same trace share
                    /// the same `trace_id`. The ID is a 16-byte array. An ID with all zeroes OR
                    /// of length other than 16 bytes is considered invalid (empty string in OTLP/JSON
                    /// is zero-length and thus is also invalid).
                    ///
                    /// This field is required.
                    pub r#trace_id: ::std::vec::Vec<u8>,
                    /// A unique identifier for a span within a trace, assigned when the span
                    /// is created. The ID is an 8-byte array. An ID with all zeroes OR of length
                    /// other than 8 bytes is considered invalid (empty string in OTLP/JSON
                    /// is zero-length and thus is also invalid).
                    ///
                    /// This field is required.
                    pub r#span_id: ::std::vec::Vec<u8>,
                    /// trace_state conveys information about request position in multiple distributed tracing graphs.
                    /// It is a trace_state in w3c-trace-context format: https://www.w3.org/TR/trace-context/#tracestate-header
                    /// See also https://github.com/w3c/distributed-tracing for more details about this field.
                    pub r#trace_state: ::std::string::String,
                    /// The `span_id` of this span's parent span. If this is a root span, then this
                    /// field must be empty. The ID is an 8-byte array.
                    pub r#parent_span_id: ::std::vec::Vec<u8>,
                    /// Flags, a bit field.
                    ///
                    /// Bits 0-7 (8 least significant bits) are the trace flags as defined in W3C Trace
                    /// Context specification. To read the 8-bit W3C trace flag, use
                    /// `flags & SPAN_FLAGS_TRACE_FLAGS_MASK`.
                    ///
                    /// See https://www.w3.org/TR/trace-context-2/#trace-flags for the flag definitions.
                    ///
                    /// Bits 8 and 9 represent the 3 states of whether a span's parent
                    /// is remote. The states are (unknown, is not remote, is remote).
                    /// To read whether the value is known, use `(flags & SPAN_FLAGS_CONTEXT_HAS_IS_REMOTE_MASK) != 0`.
                    /// To read whether the span is remote, use `(flags & SPAN_FLAGS_CONTEXT_IS_REMOTE_MASK) != 0`.
                    ///
                    /// When creating span messages, if the message is logically forwarded from another source
                    /// with an equivalent flags fields (i.e., usually another OTLP span message), the field SHOULD
                    /// be copied as-is. If creating from a source that does not have an equivalent flags field
                    /// (such as a runtime representation of an OpenTelemetry span), the high 22 bits MUST
                    /// be set to zero.
                    /// Readers MUST NOT assume that bits 10-31 (22 most significant bits) will be zero.
                    ///
                    /// [Optional].
                    pub r#flags: u32,
                    /// A description of the span's operation.
                    ///
                    /// For example, the name can be a qualified method name or a file name
                    /// and a line number where the operation is called. A best practice is to use
                    /// the same display name at the same call point in an application.
                    /// This makes it easier to correlate spans in different traces.
                    ///
                    /// This field is semantically required to be set to non-empty string.
                    /// Empty value is equivalent to an unknown span name.
                    ///
                    /// This field is required.
                    pub r#name: ::std::string::String,
                    /// Distinguishes between spans generated in a particular context. For example,
                    /// two spans with the same name may be distinguished using `CLIENT` (caller)
                    /// and `SERVER` (callee) to identify queueing latency associated with the span.
                    pub r#kind: Span_::SpanKind,
                    /// The start time of the span. On the client side, this is the time
                    /// kept by the local machine where the span execution starts. On the server side, this
                    /// is the time when the server's application handler starts running.
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
                    ///
                    /// This field is semantically required and it is expected that end_time >= start_time.
                    pub r#start_time_unix_nano: u64,
                    /// The end time of the span. On the client side, this is the time
                    /// kept by the local machine where the span execution ends. On the server side, this
                    /// is the time when the server application handler stops running.
                    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
                    ///
                    /// This field is semantically required and it is expected that end_time >= start_time.
                    pub r#end_time_unix_nano: u64,
                    /// A collection of key/value pairs. Note, global attributes
                    /// like server name can be set using the resource API. Examples of attributes:
                    ///
                    ///     "/http/user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36"
                    ///     "/http/server_latency": 300
                    ///     "example.com/myattribute": true
                    ///     "example.com/score": 10.239
                    ///
                    /// Attribute keys MUST be unique (it is not allowed to have more than one
                    /// attribute with the same key).
                    /// The behavior of software that receives duplicated keys can be unpredictable.
                    pub r#attributes: ::std::vec::Vec<
                        super::super::common_::v1_::KeyValue,
                    >,
                    /// The number of attributes that were discarded. Attributes
                    /// can be discarded because their keys are too long or because there are too many
                    /// attributes. If this value is 0, then no attributes were dropped.
                    pub r#dropped_attributes_count: u32,
                    /// A collection of Event items.
                    pub r#events: ::std::vec::Vec<Span_::Event>,
                    /// The number of dropped events. If the value is 0, then no
                    /// events were dropped.
                    pub r#dropped_events_count: u32,
                    /// A collection of Links, which are references from this span to a span
                    /// in the same or different trace.
                    pub r#links: ::std::vec::Vec<Span_::Link>,
                    /// The number of dropped links after the maximum size was
                    /// enforced. If this value is 0, then no links were dropped.
                    pub r#dropped_links_count: u32,
                    /// An optional final status for this span. Semantically when Status isn't set, it means
                    /// span's status code is unset, i.e. assume STATUS_CODE_UNSET (code = 0).
                    ///
                    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
                    pub r#status: Status,
                    /// Tracks presence of optional and message fields
                    pub _has: Span_::_Hazzer,
                }
                impl ::core::cmp::PartialEq for Span {
                    fn eq(&self, other: &Self) -> bool {
                        let mut ret = true;
                        ret &= (self.r#trace_id == other.r#trace_id);
                        ret &= (self.r#span_id == other.r#span_id);
                        ret &= (self.r#trace_state == other.r#trace_state);
                        ret &= (self.r#parent_span_id == other.r#parent_span_id);
                        ret &= (self.r#flags == other.r#flags);
                        ret &= (self.r#name == other.r#name);
                        ret &= (self.r#kind == other.r#kind);
                        ret
                            &= (self.r#start_time_unix_nano
                                == other.r#start_time_unix_nano);
                        ret &= (self.r#end_time_unix_nano == other.r#end_time_unix_nano);
                        ret &= (self.r#attributes == other.r#attributes);
                        ret
                            &= (self.r#dropped_attributes_count
                                == other.r#dropped_attributes_count);
                        ret &= (self.r#events == other.r#events);
                        ret
                            &= (self.r#dropped_events_count
                                == other.r#dropped_events_count);
                        ret &= (self.r#links == other.r#links);
                        ret
                            &= (self.r#dropped_links_count
                                == other.r#dropped_links_count);
                        ret &= (self.r#status() == other.r#status());
                        ret
                    }
                }
                impl Span {
                    /// Return a reference to `trace_id`
                    #[inline]
                    pub fn r#trace_id(&self) -> &::std::vec::Vec<u8> {
                        &self.r#trace_id
                    }
                    /// Return a mutable reference to `trace_id`
                    #[inline]
                    pub fn mut_trace_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                        &mut self.r#trace_id
                    }
                    /// Set the value of `trace_id`
                    #[inline]
                    pub fn set_trace_id(
                        &mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> &mut Self {
                        self.r#trace_id = value.into();
                        self
                    }
                    /// Builder method that sets the value of `trace_id`. Useful for initializing the message.
                    #[inline]
                    pub fn init_trace_id(mut self, value: ::std::vec::Vec<u8>) -> Self {
                        self.r#trace_id = value.into();
                        self
                    }
                    /// Return a reference to `span_id`
                    #[inline]
                    pub fn r#span_id(&self) -> &::std::vec::Vec<u8> {
                        &self.r#span_id
                    }
                    /// Return a mutable reference to `span_id`
                    #[inline]
                    pub fn mut_span_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                        &mut self.r#span_id
                    }
                    /// Set the value of `span_id`
                    #[inline]
                    pub fn set_span_id(
                        &mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> &mut Self {
                        self.r#span_id = value.into();
                        self
                    }
                    /// Builder method that sets the value of `span_id`. Useful for initializing the message.
                    #[inline]
                    pub fn init_span_id(mut self, value: ::std::vec::Vec<u8>) -> Self {
                        self.r#span_id = value.into();
                        self
                    }
                    /// Return a reference to `trace_state`
                    #[inline]
                    pub fn r#trace_state(&self) -> &::std::string::String {
                        &self.r#trace_state
                    }
                    /// Return a mutable reference to `trace_state`
                    #[inline]
                    pub fn mut_trace_state(&mut self) -> &mut ::std::string::String {
                        &mut self.r#trace_state
                    }
                    /// Set the value of `trace_state`
                    #[inline]
                    pub fn set_trace_state(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#trace_state = value.into();
                        self
                    }
                    /// Builder method that sets the value of `trace_state`. Useful for initializing the message.
                    #[inline]
                    pub fn init_trace_state(
                        mut self,
                        value: ::std::string::String,
                    ) -> Self {
                        self.r#trace_state = value.into();
                        self
                    }
                    /// Return a reference to `parent_span_id`
                    #[inline]
                    pub fn r#parent_span_id(&self) -> &::std::vec::Vec<u8> {
                        &self.r#parent_span_id
                    }
                    /// Return a mutable reference to `parent_span_id`
                    #[inline]
                    pub fn mut_parent_span_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                        &mut self.r#parent_span_id
                    }
                    /// Set the value of `parent_span_id`
                    #[inline]
                    pub fn set_parent_span_id(
                        &mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> &mut Self {
                        self.r#parent_span_id = value.into();
                        self
                    }
                    /// Builder method that sets the value of `parent_span_id`. Useful for initializing the message.
                    #[inline]
                    pub fn init_parent_span_id(
                        mut self,
                        value: ::std::vec::Vec<u8>,
                    ) -> Self {
                        self.r#parent_span_id = value.into();
                        self
                    }
                    /// Return a reference to `flags`
                    #[inline]
                    pub fn r#flags(&self) -> &u32 {
                        &self.r#flags
                    }
                    /// Return a mutable reference to `flags`
                    #[inline]
                    pub fn mut_flags(&mut self) -> &mut u32 {
                        &mut self.r#flags
                    }
                    /// Set the value of `flags`
                    #[inline]
                    pub fn set_flags(&mut self, value: u32) -> &mut Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Builder method that sets the value of `flags`. Useful for initializing the message.
                    #[inline]
                    pub fn init_flags(mut self, value: u32) -> Self {
                        self.r#flags = value.into();
                        self
                    }
                    /// Return a reference to `name`
                    #[inline]
                    pub fn r#name(&self) -> &::std::string::String {
                        &self.r#name
                    }
                    /// Return a mutable reference to `name`
                    #[inline]
                    pub fn mut_name(&mut self) -> &mut ::std::string::String {
                        &mut self.r#name
                    }
                    /// Set the value of `name`
                    #[inline]
                    pub fn set_name(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#name = value.into();
                        self
                    }
                    /// Builder method that sets the value of `name`. Useful for initializing the message.
                    #[inline]
                    pub fn init_name(mut self, value: ::std::string::String) -> Self {
                        self.r#name = value.into();
                        self
                    }
                    /// Return a reference to `kind`
                    #[inline]
                    pub fn r#kind(&self) -> &Span_::SpanKind {
                        &self.r#kind
                    }
                    /// Return a mutable reference to `kind`
                    #[inline]
                    pub fn mut_kind(&mut self) -> &mut Span_::SpanKind {
                        &mut self.r#kind
                    }
                    /// Set the value of `kind`
                    #[inline]
                    pub fn set_kind(&mut self, value: Span_::SpanKind) -> &mut Self {
                        self.r#kind = value.into();
                        self
                    }
                    /// Builder method that sets the value of `kind`. Useful for initializing the message.
                    #[inline]
                    pub fn init_kind(mut self, value: Span_::SpanKind) -> Self {
                        self.r#kind = value.into();
                        self
                    }
                    /// Return a reference to `start_time_unix_nano`
                    #[inline]
                    pub fn r#start_time_unix_nano(&self) -> &u64 {
                        &self.r#start_time_unix_nano
                    }
                    /// Return a mutable reference to `start_time_unix_nano`
                    #[inline]
                    pub fn mut_start_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#start_time_unix_nano
                    }
                    /// Set the value of `start_time_unix_nano`
                    #[inline]
                    pub fn set_start_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `start_time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_start_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#start_time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `end_time_unix_nano`
                    #[inline]
                    pub fn r#end_time_unix_nano(&self) -> &u64 {
                        &self.r#end_time_unix_nano
                    }
                    /// Return a mutable reference to `end_time_unix_nano`
                    #[inline]
                    pub fn mut_end_time_unix_nano(&mut self) -> &mut u64 {
                        &mut self.r#end_time_unix_nano
                    }
                    /// Set the value of `end_time_unix_nano`
                    #[inline]
                    pub fn set_end_time_unix_nano(&mut self, value: u64) -> &mut Self {
                        self.r#end_time_unix_nano = value.into();
                        self
                    }
                    /// Builder method that sets the value of `end_time_unix_nano`. Useful for initializing the message.
                    #[inline]
                    pub fn init_end_time_unix_nano(mut self, value: u64) -> Self {
                        self.r#end_time_unix_nano = value.into();
                        self
                    }
                    /// Return a reference to `dropped_attributes_count`
                    #[inline]
                    pub fn r#dropped_attributes_count(&self) -> &u32 {
                        &self.r#dropped_attributes_count
                    }
                    /// Return a mutable reference to `dropped_attributes_count`
                    #[inline]
                    pub fn mut_dropped_attributes_count(&mut self) -> &mut u32 {
                        &mut self.r#dropped_attributes_count
                    }
                    /// Set the value of `dropped_attributes_count`
                    #[inline]
                    pub fn set_dropped_attributes_count(
                        &mut self,
                        value: u32,
                    ) -> &mut Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `dropped_attributes_count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_dropped_attributes_count(mut self, value: u32) -> Self {
                        self.r#dropped_attributes_count = value.into();
                        self
                    }
                    /// Return a reference to `dropped_events_count`
                    #[inline]
                    pub fn r#dropped_events_count(&self) -> &u32 {
                        &self.r#dropped_events_count
                    }
                    /// Return a mutable reference to `dropped_events_count`
                    #[inline]
                    pub fn mut_dropped_events_count(&mut self) -> &mut u32 {
                        &mut self.r#dropped_events_count
                    }
                    /// Set the value of `dropped_events_count`
                    #[inline]
                    pub fn set_dropped_events_count(&mut self, value: u32) -> &mut Self {
                        self.r#dropped_events_count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `dropped_events_count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_dropped_events_count(mut self, value: u32) -> Self {
                        self.r#dropped_events_count = value.into();
                        self
                    }
                    /// Return a reference to `dropped_links_count`
                    #[inline]
                    pub fn r#dropped_links_count(&self) -> &u32 {
                        &self.r#dropped_links_count
                    }
                    /// Return a mutable reference to `dropped_links_count`
                    #[inline]
                    pub fn mut_dropped_links_count(&mut self) -> &mut u32 {
                        &mut self.r#dropped_links_count
                    }
                    /// Set the value of `dropped_links_count`
                    #[inline]
                    pub fn set_dropped_links_count(&mut self, value: u32) -> &mut Self {
                        self.r#dropped_links_count = value.into();
                        self
                    }
                    /// Builder method that sets the value of `dropped_links_count`. Useful for initializing the message.
                    #[inline]
                    pub fn init_dropped_links_count(mut self, value: u32) -> Self {
                        self.r#dropped_links_count = value.into();
                        self
                    }
                    /// Return a reference to `status` as an `Option`
                    #[inline]
                    pub fn r#status(&self) -> ::core::option::Option<&Status> {
                        self._has.r#status().then_some(&self.r#status)
                    }
                    /// Set the value and presence of `status`
                    #[inline]
                    pub fn set_status(&mut self, value: Status) -> &mut Self {
                        self._has.set_status();
                        self.r#status = value.into();
                        self
                    }
                    /// Return a mutable reference to `status` as an `Option`
                    #[inline]
                    pub fn mut_status(&mut self) -> ::core::option::Option<&mut Status> {
                        self._has.r#status().then_some(&mut self.r#status)
                    }
                    /// Clear the presence of `status`
                    #[inline]
                    pub fn clear_status(&mut self) -> &mut Self {
                        self._has.clear_status();
                        self
                    }
                    /// Take the value of `status` and clear its presence
                    #[inline]
                    pub fn take_status(&mut self) -> ::core::option::Option<Status> {
                        let val = self
                            ._has
                            .r#status()
                            .then(|| ::core::mem::take(&mut self.r#status));
                        self._has.clear_status();
                        val
                    }
                    /// Builder method that sets the value of `status`. Useful for initializing the message.
                    #[inline]
                    pub fn init_status(mut self, value: Status) -> Self {
                        self.set_status(value);
                        self
                    }
                }
                impl ::micropb::MessageDecode for Span {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.Span.trace_id) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.Span.span_id) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.Span.trace_state) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.Span.parent_span_id) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(4usize), | size | size + 2usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.Span.name) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(Span_::SpanKind::_MAX_SIZE), |
                            size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(8usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.trace.v1.Span.attributes) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.trace.v1.Span.events) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::core::result::Result::<
                            usize,
                            &'static str,
                        >::Err(
                            "(.opentelemetry.proto.trace.v1.Span.links) unbounded vec",
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(5usize), | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::micropb::const_map!(< Status as ::micropb::MessageEncode >
                            ::MAX_SIZE, | size |
                            ::micropb::size::sizeof_len_record(size)), | size | size +
                            1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
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
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            for (i, val_ref) in self.r#events.iter().enumerate() {
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
                            for (i, val_ref) in self.r#links.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#status()
                            {
                                encoder.encode_varint32(122u32)?;
                                val_ref.encode_len_delimited(encoder)?;
                            }
                        }
                        Ok(())
                    }
                    fn compute_size(&self) -> usize {
                        use ::micropb::{PbMap, FieldEncode};
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
                            for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            for (i, val_ref) in self.r#events.iter().enumerate() {
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
                            for (i, val_ref) in self.r#links.iter().enumerate() {
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
                            if let ::core::option::Option::Some(val_ref) = self
                                .r#status()
                            {
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
                /// Inner types for `Span`
                pub mod Span_ {
                    /// Event is a time-stamped annotation of the span, consisting of user-supplied
                    /// text description and key-value pairs.
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct Event {
                        /// The time the event occurred.
                        pub r#time_unix_nano: u64,
                        /// The name of the event.
                        /// This field is semantically required to be set to non-empty string.
                        pub r#name: ::std::string::String,
                        /// A collection of attribute key/value pairs on the event.
                        /// Attribute keys MUST be unique (it is not allowed to have more than one
                        /// attribute with the same key).
                        /// The behavior of software that receives duplicated keys can be unpredictable.
                        pub r#attributes: ::std::vec::Vec<
                            super::super::super::common_::v1_::KeyValue,
                        >,
                        /// The number of dropped attributes. If the value is 0,
                        /// then no attributes were dropped.
                        pub r#dropped_attributes_count: u32,
                    }
                    impl Event {
                        /// Return a reference to `time_unix_nano`
                        #[inline]
                        pub fn r#time_unix_nano(&self) -> &u64 {
                            &self.r#time_unix_nano
                        }
                        /// Return a mutable reference to `time_unix_nano`
                        #[inline]
                        pub fn mut_time_unix_nano(&mut self) -> &mut u64 {
                            &mut self.r#time_unix_nano
                        }
                        /// Set the value of `time_unix_nano`
                        #[inline]
                        pub fn set_time_unix_nano(&mut self, value: u64) -> &mut Self {
                            self.r#time_unix_nano = value.into();
                            self
                        }
                        /// Builder method that sets the value of `time_unix_nano`. Useful for initializing the message.
                        #[inline]
                        pub fn init_time_unix_nano(mut self, value: u64) -> Self {
                            self.r#time_unix_nano = value.into();
                            self
                        }
                        /// Return a reference to `name`
                        #[inline]
                        pub fn r#name(&self) -> &::std::string::String {
                            &self.r#name
                        }
                        /// Return a mutable reference to `name`
                        #[inline]
                        pub fn mut_name(&mut self) -> &mut ::std::string::String {
                            &mut self.r#name
                        }
                        /// Set the value of `name`
                        #[inline]
                        pub fn set_name(
                            &mut self,
                            value: ::std::string::String,
                        ) -> &mut Self {
                            self.r#name = value.into();
                            self
                        }
                        /// Builder method that sets the value of `name`. Useful for initializing the message.
                        #[inline]
                        pub fn init_name(
                            mut self,
                            value: ::std::string::String,
                        ) -> Self {
                            self.r#name = value.into();
                            self
                        }
                        /// Return a reference to `dropped_attributes_count`
                        #[inline]
                        pub fn r#dropped_attributes_count(&self) -> &u32 {
                            &self.r#dropped_attributes_count
                        }
                        /// Return a mutable reference to `dropped_attributes_count`
                        #[inline]
                        pub fn mut_dropped_attributes_count(&mut self) -> &mut u32 {
                            &mut self.r#dropped_attributes_count
                        }
                        /// Set the value of `dropped_attributes_count`
                        #[inline]
                        pub fn set_dropped_attributes_count(
                            &mut self,
                            value: u32,
                        ) -> &mut Self {
                            self.r#dropped_attributes_count = value.into();
                            self
                        }
                        /// Builder method that sets the value of `dropped_attributes_count`. Useful for initializing the message.
                        #[inline]
                        pub fn init_dropped_attributes_count(
                            mut self,
                            value: u32,
                        ) -> Self {
                            self.r#dropped_attributes_count = value.into();
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for Event {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
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
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(8usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.trace.v1.Event.name) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::core::result::Result::<
                                usize,
                                &'static str,
                            >::Err(
                                "(.opentelemetry.proto.trace.v1.Event.attributes) unbounded vec",
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(5usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
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
                                for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            use ::micropb::{PbMap, FieldEncode};
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
                                for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                    /// A pointer from the current span to another span in the same trace or in a
                    /// different trace. For example, this can be used in batching operations,
                    /// where a single batch handler processes multiple requests from different
                    /// traces or when the handler receives a request from a different project.
                    #[derive(Debug, Default, PartialEq, Clone)]
                    pub struct Link {
                        /// A unique identifier of a trace that this linked span is part of. The ID is a
                        /// 16-byte array.
                        pub r#trace_id: ::std::vec::Vec<u8>,
                        /// A unique identifier for the linked span. The ID is an 8-byte array.
                        pub r#span_id: ::std::vec::Vec<u8>,
                        /// The trace_state associated with the link.
                        pub r#trace_state: ::std::string::String,
                        /// A collection of attribute key/value pairs on the link.
                        /// Attribute keys MUST be unique (it is not allowed to have more than one
                        /// attribute with the same key).
                        /// The behavior of software that receives duplicated keys can be unpredictable.
                        pub r#attributes: ::std::vec::Vec<
                            super::super::super::common_::v1_::KeyValue,
                        >,
                        /// The number of dropped attributes. If the value is 0,
                        /// then no attributes were dropped.
                        pub r#dropped_attributes_count: u32,
                        /// Flags, a bit field.
                        ///
                        /// Bits 0-7 (8 least significant bits) are the trace flags as defined in W3C Trace
                        /// Context specification. To read the 8-bit W3C trace flag, use
                        /// `flags & SPAN_FLAGS_TRACE_FLAGS_MASK`.
                        ///
                        /// See https://www.w3.org/TR/trace-context-2/#trace-flags for the flag definitions.
                        ///
                        /// Bits 8 and 9 represent the 3 states of whether the link is remote.
                        /// The states are (unknown, is not remote, is remote).
                        /// To read whether the value is known, use `(flags & SPAN_FLAGS_CONTEXT_HAS_IS_REMOTE_MASK) != 0`.
                        /// To read whether the link is remote, use `(flags & SPAN_FLAGS_CONTEXT_IS_REMOTE_MASK) != 0`.
                        ///
                        /// Readers MUST NOT assume that bits 10-31 (22 most significant bits) will be zero.
                        /// When creating new spans, bits 10-31 (most-significant 22-bits) MUST be zero.
                        ///
                        /// [Optional].
                        pub r#flags: u32,
                    }
                    impl Link {
                        /// Return a reference to `trace_id`
                        #[inline]
                        pub fn r#trace_id(&self) -> &::std::vec::Vec<u8> {
                            &self.r#trace_id
                        }
                        /// Return a mutable reference to `trace_id`
                        #[inline]
                        pub fn mut_trace_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                            &mut self.r#trace_id
                        }
                        /// Set the value of `trace_id`
                        #[inline]
                        pub fn set_trace_id(
                            &mut self,
                            value: ::std::vec::Vec<u8>,
                        ) -> &mut Self {
                            self.r#trace_id = value.into();
                            self
                        }
                        /// Builder method that sets the value of `trace_id`. Useful for initializing the message.
                        #[inline]
                        pub fn init_trace_id(
                            mut self,
                            value: ::std::vec::Vec<u8>,
                        ) -> Self {
                            self.r#trace_id = value.into();
                            self
                        }
                        /// Return a reference to `span_id`
                        #[inline]
                        pub fn r#span_id(&self) -> &::std::vec::Vec<u8> {
                            &self.r#span_id
                        }
                        /// Return a mutable reference to `span_id`
                        #[inline]
                        pub fn mut_span_id(&mut self) -> &mut ::std::vec::Vec<u8> {
                            &mut self.r#span_id
                        }
                        /// Set the value of `span_id`
                        #[inline]
                        pub fn set_span_id(
                            &mut self,
                            value: ::std::vec::Vec<u8>,
                        ) -> &mut Self {
                            self.r#span_id = value.into();
                            self
                        }
                        /// Builder method that sets the value of `span_id`. Useful for initializing the message.
                        #[inline]
                        pub fn init_span_id(
                            mut self,
                            value: ::std::vec::Vec<u8>,
                        ) -> Self {
                            self.r#span_id = value.into();
                            self
                        }
                        /// Return a reference to `trace_state`
                        #[inline]
                        pub fn r#trace_state(&self) -> &::std::string::String {
                            &self.r#trace_state
                        }
                        /// Return a mutable reference to `trace_state`
                        #[inline]
                        pub fn mut_trace_state(&mut self) -> &mut ::std::string::String {
                            &mut self.r#trace_state
                        }
                        /// Set the value of `trace_state`
                        #[inline]
                        pub fn set_trace_state(
                            &mut self,
                            value: ::std::string::String,
                        ) -> &mut Self {
                            self.r#trace_state = value.into();
                            self
                        }
                        /// Builder method that sets the value of `trace_state`. Useful for initializing the message.
                        #[inline]
                        pub fn init_trace_state(
                            mut self,
                            value: ::std::string::String,
                        ) -> Self {
                            self.r#trace_state = value.into();
                            self
                        }
                        /// Return a reference to `dropped_attributes_count`
                        #[inline]
                        pub fn r#dropped_attributes_count(&self) -> &u32 {
                            &self.r#dropped_attributes_count
                        }
                        /// Return a mutable reference to `dropped_attributes_count`
                        #[inline]
                        pub fn mut_dropped_attributes_count(&mut self) -> &mut u32 {
                            &mut self.r#dropped_attributes_count
                        }
                        /// Set the value of `dropped_attributes_count`
                        #[inline]
                        pub fn set_dropped_attributes_count(
                            &mut self,
                            value: u32,
                        ) -> &mut Self {
                            self.r#dropped_attributes_count = value.into();
                            self
                        }
                        /// Builder method that sets the value of `dropped_attributes_count`. Useful for initializing the message.
                        #[inline]
                        pub fn init_dropped_attributes_count(
                            mut self,
                            value: u32,
                        ) -> Self {
                            self.r#dropped_attributes_count = value.into();
                            self
                        }
                        /// Return a reference to `flags`
                        #[inline]
                        pub fn r#flags(&self) -> &u32 {
                            &self.r#flags
                        }
                        /// Return a mutable reference to `flags`
                        #[inline]
                        pub fn mut_flags(&mut self) -> &mut u32 {
                            &mut self.r#flags
                        }
                        /// Set the value of `flags`
                        #[inline]
                        pub fn set_flags(&mut self, value: u32) -> &mut Self {
                            self.r#flags = value.into();
                            self
                        }
                        /// Builder method that sets the value of `flags`. Useful for initializing the message.
                        #[inline]
                        pub fn init_flags(mut self, value: u32) -> Self {
                            self.r#flags = value.into();
                            self
                        }
                    }
                    impl ::micropb::MessageDecode for Link {
                        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                            &mut self,
                            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                            len: usize,
                        ) -> Result<
                            (),
                            ::micropb::DecodeError<IMPL_MICROPB_READ::Error>,
                        > {
                            use ::micropb::{
                                PbBytes, PbString, PbVec, PbMap, FieldDecode,
                            };
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
                        const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                            let mut max_size = 0;
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.trace.v1.Link.trace_id) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.trace.v1.Link.span_id) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result:: < usize, & 'static str >
                                ::Err("(.opentelemetry.proto.trace.v1.Link.trace_state) unbounded string or bytes"),
                                | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::core::result::Result::<
                                usize,
                                &'static str,
                            >::Err(
                                "(.opentelemetry.proto.trace.v1.Link.attributes) unbounded vec",
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(5usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            match ::micropb::const_map!(
                                ::core::result::Result::Ok(4usize), | size | size + 1usize
                            ) {
                                ::core::result::Result::Ok(size) => {
                                    max_size += size;
                                }
                                ::core::result::Result::Err(err) => {
                                    break 'msg (::core::result::Result::<usize, _>::Err(err));
                                }
                            }
                            ::core::result::Result::Ok(max_size)
                        };
                        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                            &self,
                            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                            use ::micropb::{PbMap, FieldEncode};
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
                                for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                            use ::micropb::{PbMap, FieldEncode};
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
                                for (i, val_ref) in self.r#attributes.iter().enumerate() {
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
                    /// SpanKind is the type of span. Can be used to specify additional relationships between spans
                    /// in addition to a parent/child relationship.
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[repr(transparent)]
                    pub struct SpanKind(pub i32);
                    impl SpanKind {
                        /// Maximum encoded size of the enum
                        pub const _MAX_SIZE: usize = 10usize;
                        /// Unspecified. Do NOT use as default.
                        /// Implementations MAY assume SpanKind to be INTERNAL when receiving UNSPECIFIED.
                        pub const Unspecified: Self = Self(0);
                        /// Indicates that the span represents an internal operation within an application,
                        /// as opposed to an operation happening at the boundaries. Default value.
                        pub const Internal: Self = Self(1);
                        /// Indicates that the span covers server-side handling of an RPC or other
                        /// remote network request.
                        pub const Server: Self = Self(2);
                        /// Indicates that the span describes a request to some remote service.
                        pub const Client: Self = Self(3);
                        /// Indicates that the span describes a producer sending a message to a broker.
                        /// Unlike CLIENT and SERVER, there is often no direct critical path latency relationship
                        /// between producer and consumer spans. A PRODUCER span ends when the message was accepted
                        /// by the broker while the logical processing of the message might span a much longer time.
                        pub const Producer: Self = Self(4);
                        /// Indicates that the span describes consumer receiving a message from a broker.
                        /// Like the PRODUCER kind, there is often no direct critical path latency relationship
                        /// between producer and consumer spans.
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
                    /// Compact bitfield for tracking presence of optional and message fields
                    #[derive(Debug, Default, PartialEq, Clone, Copy)]
                    pub struct _Hazzer([u8; 1]);
                    impl _Hazzer {
                        /// New hazzer with all fields set to off
                        #[inline]
                        pub const fn _new() -> Self {
                            Self([0; 1])
                        }
                        /// Query presence of `status`
                        #[inline]
                        pub const fn r#status(&self) -> bool {
                            (self.0[0] & 1) != 0
                        }
                        /// Set presence of `status`
                        #[inline]
                        pub const fn set_status(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem |= 1;
                            self
                        }
                        /// Clear presence of `status`
                        #[inline]
                        pub const fn clear_status(&mut self) -> &mut Self {
                            let elem = &mut self.0[0];
                            *elem &= !1;
                            self
                        }
                        /// Builder method that sets the presence of `status`. Useful for initializing the Hazzer.
                        #[inline]
                        pub const fn init_status(mut self) -> Self {
                            self.set_status();
                            self
                        }
                    }
                }
                /// The Status type defines a logical error model that is suitable for different
                /// programming environments, including REST APIs and RPC APIs.
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct Status {
                    /// A developer-facing human readable error message.
                    pub r#message: ::std::string::String,
                    /// The status code.
                    pub r#code: Status_::StatusCode,
                }
                impl Status {
                    /// Return a reference to `message`
                    #[inline]
                    pub fn r#message(&self) -> &::std::string::String {
                        &self.r#message
                    }
                    /// Return a mutable reference to `message`
                    #[inline]
                    pub fn mut_message(&mut self) -> &mut ::std::string::String {
                        &mut self.r#message
                    }
                    /// Set the value of `message`
                    #[inline]
                    pub fn set_message(
                        &mut self,
                        value: ::std::string::String,
                    ) -> &mut Self {
                        self.r#message = value.into();
                        self
                    }
                    /// Builder method that sets the value of `message`. Useful for initializing the message.
                    #[inline]
                    pub fn init_message(mut self, value: ::std::string::String) -> Self {
                        self.r#message = value.into();
                        self
                    }
                    /// Return a reference to `code`
                    #[inline]
                    pub fn r#code(&self) -> &Status_::StatusCode {
                        &self.r#code
                    }
                    /// Return a mutable reference to `code`
                    #[inline]
                    pub fn mut_code(&mut self) -> &mut Status_::StatusCode {
                        &mut self.r#code
                    }
                    /// Set the value of `code`
                    #[inline]
                    pub fn set_code(&mut self, value: Status_::StatusCode) -> &mut Self {
                        self.r#code = value.into();
                        self
                    }
                    /// Builder method that sets the value of `code`. Useful for initializing the message.
                    #[inline]
                    pub fn init_code(mut self, value: Status_::StatusCode) -> Self {
                        self.r#code = value.into();
                        self
                    }
                }
                impl ::micropb::MessageDecode for Status {
                    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                        &mut self,
                        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                        len: usize,
                    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
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
                    const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                        let mut max_size = 0;
                        match ::micropb::const_map!(
                            ::core::result::Result:: < usize, & 'static str >
                            ::Err("(.opentelemetry.proto.trace.v1.Status.message) unbounded string or bytes"),
                            | size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        match ::micropb::const_map!(
                            ::core::result::Result::Ok(Status_::StatusCode::_MAX_SIZE), |
                            size | size + 1usize
                        ) {
                            ::core::result::Result::Ok(size) => {
                                max_size += size;
                            }
                            ::core::result::Result::Err(err) => {
                                break 'msg (::core::result::Result::<usize, _>::Err(err));
                            }
                        }
                        ::core::result::Result::Ok(max_size)
                    };
                    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                        &self,
                        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                        use ::micropb::{PbMap, FieldEncode};
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
                        use ::micropb::{PbMap, FieldEncode};
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
                /// Inner types for `Status`
                pub mod Status_ {
                    /// For the semantics of status codes see
                    /// https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/api.md#set-status
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[repr(transparent)]
                    pub struct StatusCode(pub i32);
                    impl StatusCode {
                        /// Maximum encoded size of the enum
                        pub const _MAX_SIZE: usize = 10usize;
                        /// The default status.
                        pub const Unset: Self = Self(0);
                        /// The Span has been validated by an Application developer or Operator to
                        /// have completed successfully.
                        pub const Ok: Self = Self(1);
                        /// The Span contains an error.
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
                /// SpanFlags represents constants used to interpret the
                /// Span.flags field, which is protobuf 'fixed32' type and is to
                /// be used as bit-fields. Each non-zero value defined in this enum is
                /// a bit-mask.  To extract the bit-field, for example, use an
                /// expression like:
                ///
                ///   (span.flags & SPAN_FLAGS_TRACE_FLAGS_MASK)
                ///
                /// See https://www.w3.org/TR/trace-context-2/#trace-flags for the flag definitions.
                ///
                /// Note that Span flags were introduced in version 1.1 of the
                /// OpenTelemetry protocol.  Older Span producers do not set this
                /// field, consequently consumers should not rely on the absence of a
                /// particular flag bit to indicate the presence of a particular feature.
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct SpanFlags(pub i32);
                impl SpanFlags {
                    /// Maximum encoded size of the enum
                    pub const _MAX_SIZE: usize = 10usize;
                    /// The zero value for the enum. Should not be used for comparisons.
                    /// Instead use bitwise "and" with the appropriate mask as shown above.
                    pub const DoNotUse: Self = Self(0);
                    /// Bits 0-7 are used for trace flags.
                    pub const TraceFlagsMask: Self = Self(255);
                    /// Bits 8 and 9 are used to indicate that the parent span or link span is remote.
                    /// Bit 8 (`HAS_IS_REMOTE`) indicates whether the value is known.
                    /// Bit 9 (`IS_REMOTE`) indicates whether the span or link is remote.
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
