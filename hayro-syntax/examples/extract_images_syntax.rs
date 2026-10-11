//! Extract images from PDF

use std::ops::Deref;

use hayro_syntax::object::MaybeRef::NotRef;
use hayro_syntax::object::MaybeRef::Ref;
use hayro_syntax::object::Name;
use hayro_syntax::object::Object;
use hayro_syntax::object::dict::keys::BITS_PER_COMPONENT;
use hayro_syntax::object::dict::keys::BPC;
use hayro_syntax::object::dict::keys::HEIGHT;
use hayro_syntax::object::dict::keys::IMAGE;
use hayro_syntax::object::dict::keys::SUBTYPE;
use hayro_syntax::object::dict::keys::W;
use hayro_syntax::object::dict::keys::H;
use hayro_syntax::object::dict::keys::WIDTH;
use hayro_syntax::object::stream::ImageDecodeParams;

fn main() {
    let data = std::fs::read(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../hayro-tests/pdfs/custom/image_rgb8.pdf"),
    )
    .unwrap();
    let pdf = hayro_syntax::Pdf::new(data).unwrap();
    let pages = pdf.pages();
    for page in pages.iter() {
        let resources = page.resources();
        let xobjects = &resources.x_objects;
        for (_name, maybe_ref) in xobjects.entries() {
            let obj = match maybe_ref {
                Ref(obj_ref) => pdf.xref().get(obj_ref.into()).unwrap(),
                NotRef(b) => b,
            };
            let Object::Stream(s) = obj else {
                continue;
            };
            let Some(subtype) = s.dict().get::<Name<'_>>(SUBTYPE) else {
                continue;
            };
            if subtype.deref() != IMAGE {
                continue;
            }

            let dict = s.dict();
            let bpc = dict
                .get::<u8>(BPC)
                .or_else(|| dict.get::<u8>(BITS_PER_COMPONENT));
            let width = dict.get::<u32>(W).or_else(|| dict.get::<u32>(WIDTH)).unwrap();
            let height = dict.get::<u32>(H).or_else(|| dict.get::<u32>(HEIGHT)).unwrap();
            let image_params = ImageDecodeParams {
                bpc,
                width,
                height,
                ..Default::default()
            };
            let img = s.decoded_image(&image_params).unwrap();

            eprintln!("image bin {:?}", img.data);
            eprintln!("image data {:?}", img.image_data);
        }
    }
}
