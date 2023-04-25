
use crate::{
    ImageFormat, OrderBy, TakeFrom, ImageFilesBuilder, files::FileProperties, image::DynamicImage
};
use std::path::Path;

pub struct ImageFiles {
    file_list: Vec<FileProperties>
}

impl ImageFiles {

    pub fn builder() -> ImageFilesBuilder {
        ImageFilesBuilder::default()
    }

    pub(crate) fn new(file_list: Vec<FileProperties>) -> Self {
        Self { file_list }
    }

    /// Get number of files in the current working set
    pub fn file_count(&self) -> usize {
        self.file_list.len()
    }

    pub fn total_size(&self) -> u64 {
        let mut total = 0;
        for file in self.file_list.iter() {
            total += file.size_bytes;
        }
        total
    }

    /// Sorts the files according to the options supplied, and truncates the set to the
    /// number of files requested by the user
    pub fn sort_and_truncate_by(
        mut self,
        number_of_files: usize,
        order_by: OrderBy,
        take_from: TakeFrom,
        reverse: bool
    ) -> Result<Self, String> {

        // Verify at least n images were found, where n is the number requested
        if self.file_list.len() < number_of_files {
            return Err(
                format!("Requested {} files, found {}", number_of_files, self.file_list.len()));
        }

        // Sort the files by the order given, putting files at the start of the vector according to
        // whether we should take from the default end (most recently updated or alphabetically
        // first) or take from the other end (oldest update or alphabetically last)
        match (order_by, take_from) {
            (OrderBy::Latest, TakeFrom::Start) => {
                self.file_list.sort_unstable_by(|a, b|
                    a.modify_time.cmp(&b.modify_time).reverse());
            },
            (OrderBy::Latest, TakeFrom::End) => {
                self.file_list.sort_unstable_by(|a, b|
                    a.modify_time.cmp(&b.modify_time));
            },
            (OrderBy::Alphabetic, TakeFrom::Start) => {
                self.file_list.sort_unstable_by(|a, b|
                    a.full_path.cmp(&b.full_path));
            },
            (OrderBy::Alphabetic, TakeFrom::End) => {
                self.file_list.sort_unstable_by(|a, b|
                    a.full_path.cmp(&b.full_path).reverse());
            }
        }
        self.file_list.truncate(number_of_files);

        // 'Natural' order of selected files based on date is oldest to newest, which is the reverse
        // of the order generated above, or alphabetically earliest to latest, which is the same
        // as the order from above
        let reverse_order = reverse ^ (order_by == OrderBy::Latest);

        // Revert to chronological order, unless the reverse order was requested
        if reverse_order {
            self.file_list.reverse();
        }

        // Return updated self
        Ok(self)
    }

    pub fn into_image_contents(self, print_info: bool) -> Result<Vec<DynamicImage>, String> {
        let mut images = Vec::with_capacity(self.file_list.len());
        for file in self.file_list {

            let path = Path::new(&file.full_path);
            let image = image::open(path)
                .map_err(|_| format!("Failed to open: {:?}", path))?;

            if print_info {
                if let Some(file_name) = path.file_name() {
                    let w = image.width();
                    let h = image.height();
                    println!(
                        "Path: {}, w: {}, h: {}, {}",
                        file_name.to_str().unwrap(),
                        w, h, crate::util::make_size_string(file.size_bytes));
                }
            }

            images.push(image);
        }

        Ok(images)
    }

    pub fn common_format_in_sources(&self) -> ImageFormat {
        if self.file_list.is_empty() {
            return ImageFormat::Unspecified;
        }
        let mut all_formats = self.file_list.iter().map(|file_data| {
            ImageFormat::infer_format(&file_data.full_path)
        });
        let first_format = all_formats.next().unwrap();
        match all_formats.all(|fmt| fmt == first_format) {
            true => first_format,
            false => ImageFormat::Unspecified
        }
    }
}
