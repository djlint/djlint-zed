//! Registers the extension with Zed. The macro generates an undocumented
//! public function, so it lives in this private module.

zed_extension_api::register_extension!(super::DjlintExtension);
