#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "mime_support"), no_std)]

/// Implementors store (or have associated) MIME types.
pub trait AsMime {
    #[cfg(feature = "mime_support")]
    fn as_mime(&self) -> mime::Mime;
}

/// Implementors store (or have associated)
#[doc = "[\"names\"](mime::Name),"]
/// i.e. parts of MIME types.
pub trait AsName<'a> {
    #[cfg(feature = "mime_support")]
    fn as_name(&self) -> mime::Name<'a>;
}

/// Implementors have an associated consant of type
#[doc = "[`mime::Mime`]"]
#[cfg(feature = "mime_support")]
pub trait MimeExt {
    const MIME: mime::Mime; 
}

/// Implementors have an associated consant of type
#[doc = "[`mime::Name`]"]
#[cfg(feature = "mime_support")]
pub trait NameExt<'a> {
    const NAME: mime::Name<'a>; 
}

macro_rules! decl_name {
    ($type:ident for $const:ident) => {
        /// Type for
        #[doc = concat!("[`mime::", stringify!($const), "`]: [`mime::Name`]")]
        pub struct $type;
        impl<'a> AsName<'a> for $type {
            #[cfg(feature = "mime_support")]
            fn as_name(&self) -> mime::Name<'a> {
                mime::$const
            }
        }

        #[cfg(feature = "mime_support")]
        impl<'a> NameExt<'a> for $type {
            const NAME: mime::Name<'a> = mime::$const;
        }
    };
}

macro_rules! decl_mime {
    ($type:ident for $const:ident) => {
        /// Type for
        #[doc = concat!("[`mime::", stringify!($const), "`]: [`mime::Mime`]")]
        pub struct $type;
        impl AsMime for $type {
            #[cfg(feature = "mime_support")]
            fn as_mime(&self) -> mime::Mime {
                mime::$const
            }
        }

        #[cfg(feature = "mime_support")]
        impl MimeExt for $type {
            const MIME: mime::Mime = mime::$const;
        }
    };
}

decl_name!(Application for APPLICATION);
decl_name!(Audio for AUDIO);
decl_name!(Basic for BASIC);
decl_name!(Bmp for BMP);
decl_name!(Boundary for BOUNDARY);
decl_name!(Charset for CHARSET);
decl_name!(Css for CSS);
decl_name!(Csv for CSV);
decl_name!(EventStream for EVENT_STREAM);
decl_name!(Font for FONT);
decl_name!(FormData for FORM_DATA);
decl_name!(Gif for GIF);
decl_name!(Html for HTML);
decl_name!(Image for IMAGE);
decl_name!(JavaScript for JAVASCRIPT);
decl_name!(Jpeg for JPEG);
decl_name!(Json for JSON);
decl_name!(Message for MESSAGE);
decl_name!(Model for MODEL);
decl_name!(Mp4 for MP4);
decl_name!(Mpeg for MPEG);
decl_name!(Msgpack for MSGPACK);
decl_name!(Multipart for MULTIPART);
decl_name!(OctetStream for OCTET_STREAM);
decl_name!(Ogg for OGG);
decl_name!(Pdf for PDF);
decl_name!(Plain for PLAIN);
decl_name!(Png for PNG);
decl_name!(Star for STAR);
decl_name!(Svg for SVG);
decl_name!(Text for TEXT);
decl_name!(Utf8 for UTF_8);
decl_name!(VCard for VCARD);
decl_name!(Video for VIDEO);
decl_name!(Woff for WOFF);
decl_name!(Woff2 for WOFF2);
decl_name!(WwwFormUrlEncoded for WWW_FORM_URLENCODED);
decl_name!(Xml for XML);

decl_mime!(ApplicationJavaScript for APPLICATION_JAVASCRIPT);
decl_mime!(ApplicationJavaScriptUtf8 for APPLICATION_JAVASCRIPT_UTF_8);
decl_mime!(ApplicationJson for APPLICATION_JSON);
decl_mime!(ApplicationMsgpack for APPLICATION_MSGPACK);
decl_mime!(ApplicationOctetStream for APPLICATION_OCTET_STREAM);
decl_mime!(ApplicationPdf for APPLICATION_PDF);
decl_mime!(ApplicationWwwFormUrlEncoded for APPLICATION_WWW_FORM_URLENCODED);
decl_mime!(FontWoff for FONT_WOFF);
decl_mime!(FontWoff2 for FONT_WOFF2);
decl_mime!(ImageBmp for IMAGE_BMP);
decl_mime!(ImageGif for IMAGE_GIF);
decl_mime!(ImageJpeg for IMAGE_JPEG);
decl_mime!(ImagePng for IMAGE_PNG);
decl_mime!(ImageStar for IMAGE_STAR);
decl_mime!(ImageSvg for IMAGE_SVG);
decl_mime!(MultipartFormData for MULTIPART_FORM_DATA);
decl_mime!(StarStar for STAR_STAR);
decl_mime!(TextCss for TEXT_CSS);
decl_mime!(TextCssUtf8 for TEXT_CSS_UTF_8);
decl_mime!(TextCsv for TEXT_CSV);
decl_mime!(TextCsvUtf8 for TEXT_CSV_UTF_8);
decl_mime!(TextEventStream for TEXT_EVENT_STREAM);
decl_mime!(TextHtml for TEXT_HTML);
decl_mime!(TextHtmlUtf8 for TEXT_HTML_UTF_8);
decl_mime!(TextJavaScript for TEXT_JAVASCRIPT);
decl_mime!(TextPlain for TEXT_PLAIN);
decl_mime!(TextPlainUtf8 for TEXT_PLAIN_UTF_8);
decl_mime!(TextStar for TEXT_STAR);
decl_mime!(TextTabSeparatedValues for TEXT_TAB_SEPARATED_VALUES);
decl_mime!(TextTabSeparatedValuesUtf8 for TEXT_TAB_SEPARATED_VALUES_UTF_8);
decl_mime!(TextVcard for TEXT_VCARD);
decl_mime!(TextXml for TEXT_XML);
