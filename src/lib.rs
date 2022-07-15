#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "mime_support", feature = "evcxr_support")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

/// Every its implementor has an associated string slice representation
/// of the [MIME type] known at compile time.
/// 
/// [MIME type]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
pub trait MimeStrExt: Mime {
    const MIME_STR: &'static str;
}

/// Its implementors are [MIME types]
/// 
/// [MIME types]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
#[cfg(feature = "evcxr_support")]
pub trait Mime: core::fmt::Display + Into<String> {}
#[cfg(not(feature = "evcxr_support"))]
pub trait Mime: core::fmt::Display {}

/// Methods and traits for working with [MIME types] from [`mime`].
/// 
/// Due to implementation, addition of new constants of types
/// [`mime::Mime`] and [`mime::Name`] is ~~impossible~~ suboptimal.
/// 
/// The implementation depends on [`__Atoms`] enum
/// because of the [reliance] on `source` field of [`mime::Mime`].
/// 
/// [`__Atoms`]: https://github.com/hyperium/mime/blob/e3e7444ca607ff87cd1475455c26876b936af77a/src/lib.rs#L543-L548
/// [reliance]: https://github.com/hyperium/mime/blob/e3e7444ca607ff87cd1475455c26876b936af77a/src/lib.rs#L589
/// [MIME types]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
#[cfg(feature = "mime_support")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "mime_support")))]
pub mod mime_support {
    use mime::{Mime,Name};
    
    /// Implementors store (or have associated) [MIME types].
    /// 
    /// [MIME types]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
    pub trait AsMime {
        fn as_mime(&self) -> Mime;
    }

    /// Implementors store (or have associated)
    #[doc = "[\"names\"](mime::Name),"]
    /// i.e. parts of [MIME types].
    /// 
    /// [MIME types]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
    pub trait AsName<'a> {
        fn as_name(&self) -> Name<'a>;
    }

    /// Implementors have an associated consant of type
    #[doc = "[`mime::Mime`]"]
    pub trait MimeExt {
        const MIME: mime::Mime; 
    }

    /// Implementors have an associated consant of type
    #[doc = "[`mime::Name`]"]
    pub trait NameExt<'a> {
        const NAME: mime::Name<'a>; 
    }
}

/// Extra traits, structures for [MIME types] supported by [`evcxr`].
/// 
/// [`evcxr`]: https://github.com/google/evcxr
/// [MIME types]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
#[cfg(feature = "evcxr_support")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "evcxr_support")))]
pub mod evcxr_support {
    use super::MimeStrExt;
    
    macro_rules! decl_mime {
        ($type:ident as $str:literal) => {
            /// Type for 
            #[doc = concat!("`", $str, "`")]
            /// MIME type.
            pub struct $type;

            impl MimeStrExt for $type {
                const MIME_STR: &'static str = $str;
            }

            impl core::fmt::Display for $type {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    f.write_str(Self::MIME_STR)
                }
            }

            impl From<$type> for String {
                fn from(_mime: $type) -> Self {
                    <$type>::MIME_STR.to_string()
                }
            }

            impl super::Mime for $type {}
        };
    }

    decl_mime!(TextMarkdownUtf8 as "text/markdown; charset=utf-8");
    pub use super::TextHtml;
}

macro_rules! decl_name {
    ($type:ident for $const:ident as $str:literal) => {
        /// Type for
        #[doc = concat!("[`mime::", stringify!($const), "`]: [`mime::Name`]")]
        pub struct $type;

        impl core::fmt::Display for $type {
            #[cfg(not(feature = "mime_support"))]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $str)
            }

            #[cfg(feature = "mime_support")]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", mime::$const)
            }
        }

        #[cfg(feature = "mime_support")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "mime_support")))]
        impl<'a> mime_support::AsName<'a> for $type {
            fn as_name(&self) -> mime::Name<'a> {
                mime::$const
            }
        }

        #[cfg(feature = "mime_support")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "mime_support")))]
        impl<'a> mime_support::NameExt<'a> for $type {
            const NAME: mime::Name<'a> = mime::$const;
        }
    };
}

macro_rules! decl_mime {
    ($type:ident for $const:ident as $str:literal) => {
        /// Type for
        #[doc = concat!("[`mime::", stringify!($const), "`]: [`mime::Mime`]")]
        pub struct $type;
        
        impl core::fmt::Display for $type {
            #[cfg(not(feature = "mime_support"))]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $str)
            }

            #[cfg(feature = "mime_support")]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", mime::$const)
            }
        }

        #[cfg(feature = "evcxr_support")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "evcxr_support")))]
        impl From<$type> for String {
            #[cfg(not(feature = "mime_support"))]
            fn from(_mime: $type) -> Self {
                $str.to_string()
            }
            #[cfg(feature = "mime_support")]
            fn from(_mime: $type) -> Self {
                mime::$const.to_string()
            }
        }

        #[cfg(feature = "mime_support")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "mime_support")))]
        impl mime_support::AsMime for $type {
            fn as_mime(&self) -> mime::Mime {
                mime::$const
            }
        }

        #[cfg(feature = "mime_support")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "mime_support")))]
        impl mime_support::MimeExt for $type {
            const MIME: mime::Mime = mime::$const;
        }

        impl Mime for $type {}

        impl MimeStrExt for $type {
            const MIME_STR: &'static str = $str;
        }
    };
}

decl_name!(Application for APPLICATION as "application");
decl_name!(Audio for AUDIO as "audio");
decl_name!(Basic for BASIC as "basic");
decl_name!(Bmp for BMP as "bmp");
decl_name!(Boundary for BOUNDARY as "boundary");
decl_name!(Charset for CHARSET as "charset");
decl_name!(Css for CSS as "css");
decl_name!(Csv for CSV as "csv");
decl_name!(EventStream for EVENT_STREAM as "event-stream");
decl_name!(Font for FONT as "font");
decl_name!(FormData for FORM_DATA as "form-data");
decl_name!(Gif for GIF as "gif");
decl_name!(Html for HTML as "html");
decl_name!(Image for IMAGE as "image");
decl_name!(JavaScript for JAVASCRIPT as "javascript");
decl_name!(Jpeg for JPEG as "jpeg");
decl_name!(Json for JSON as "json");
decl_name!(Message for MESSAGE as "message");
decl_name!(Model for MODEL as "model");
decl_name!(Mp4 for MP4 as "mp4");
decl_name!(Mpeg for MPEG as "mpeg");
decl_name!(Msgpack for MSGPACK as "msgpack");
decl_name!(Multipart for MULTIPART as "multipart");
decl_name!(OctetStream for OCTET_STREAM as "octet-stream");
decl_name!(Ogg for OGG as "ogg");
decl_name!(Pdf for PDF as "pdf");
decl_name!(Plain for PLAIN as "plain");
decl_name!(Png for PNG as "png");
decl_name!(Star for STAR as "star");
decl_name!(Svg for SVG as "svg");
decl_name!(Text for TEXT as "text");
decl_name!(Utf8 for UTF_8 as "utf-8");
decl_name!(VCard for VCARD as "vcard");
decl_name!(Video for VIDEO as "video");
decl_name!(Woff for WOFF as "woff");
decl_name!(Woff2 for WOFF2 as "woff2");
decl_name!(WwwFormUrlEncoded for WWW_FORM_URLENCODED as "x-www-form-urlencoded");
decl_name!(Xml for XML as "xml");

decl_mime!(ApplicationJavaScript for APPLICATION_JAVASCRIPT as "application/javascript");
decl_mime!(ApplicationJavaScriptUtf8 for APPLICATION_JAVASCRIPT_UTF_8 as "application/javascript; charset=utf-8");
decl_mime!(ApplicationJson for APPLICATION_JSON as "application/json");
decl_mime!(ApplicationMsgpack for APPLICATION_MSGPACK as "application/msgpack");
decl_mime!(ApplicationOctetStream for APPLICATION_OCTET_STREAM as "application/octet-stream");
decl_mime!(ApplicationPdf for APPLICATION_PDF as "application/pdf");
decl_mime!(ApplicationWwwFormUrlEncoded for APPLICATION_WWW_FORM_URLENCODED as "application/x-www-form-urlencoded");
decl_mime!(FontWoff for FONT_WOFF as "font/woff");
decl_mime!(FontWoff2 for FONT_WOFF2 as "font/woff2");
decl_mime!(ImageBmp for IMAGE_BMP as "image/bmp");
decl_mime!(ImageGif for IMAGE_GIF as "image/gif");
decl_mime!(ImageJpeg for IMAGE_JPEG as "image/jpeg");
decl_mime!(ImagePng for IMAGE_PNG as "image/png");
decl_mime!(ImageStar for IMAGE_STAR as "image/*");
decl_mime!(ImageSvg for IMAGE_SVG as "image/svg+xml");
decl_mime!(MultipartFormData for MULTIPART_FORM_DATA as "multipart/form-data");
decl_mime!(StarStar for STAR_STAR as "*/*");
decl_mime!(TextCss for TEXT_CSS as "text/css");
decl_mime!(TextCssUtf8 for TEXT_CSS_UTF_8 as "text/css; charset=utf-8");
decl_mime!(TextCsv for TEXT_CSV as "text/csv");
decl_mime!(TextCsvUtf8 for TEXT_CSV_UTF_8 as "text/csv; charset=utf-8");
decl_mime!(TextEventStream for TEXT_EVENT_STREAM as "text/event-stream");
decl_mime!(TextHtml for TEXT_HTML as "text/html");
decl_mime!(TextHtmlUtf8 for TEXT_HTML_UTF_8 as "text/html; charset=utf-8");
decl_mime!(TextJavaScript for TEXT_JAVASCRIPT as "text/javascript");
decl_mime!(TextPlain for TEXT_PLAIN as "text/plain");
decl_mime!(TextPlainUtf8 for TEXT_PLAIN_UTF_8 as "text/plain; charset=utf-8");
decl_mime!(TextStar for TEXT_STAR as "text/*");
decl_mime!(TextTabSeparatedValues for TEXT_TAB_SEPARATED_VALUES as "text/tab-separated-values");
decl_mime!(TextTabSeparatedValuesUtf8 for TEXT_TAB_SEPARATED_VALUES_UTF_8 as "text/tab-separated-values; charset=utf-8");
decl_mime!(TextVcard for TEXT_VCARD as "text/vcard");
decl_mime!(TextXml for TEXT_XML as "text/xml");