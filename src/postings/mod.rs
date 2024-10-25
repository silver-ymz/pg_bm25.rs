mod reader;
mod serializer;
mod writer;

use bytemuck::{Pod, Zeroable};
pub use reader::{InvertedReader, PostingReader, TermInfoReader};
pub use serializer::InvertedSerializer;
pub use writer::PostingsWriter;

use crate::utils::compress_block::compressed_block_size;

pub const TERMINATED_DOC: u32 = u32::MAX;

pub const COMPRESSION_BLOCK_SIZE: usize =
    <bitpacking::BitPacker4x as bitpacking::BitPacker>::BLOCK_LEN;

#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct TermInfo {
    pub doc_count: u32,
    pub postings_blkno: pgrx::pg_sys::BlockNumber,
}

impl Default for TermInfo {
    fn default() -> Self {
        Self {
            doc_count: 0,
            postings_blkno: pgrx::pg_sys::InvalidBlockNumber,
        }
    }
}

unsafe impl Zeroable for TermInfo {}
unsafe impl Pod for TermInfo {}

#[repr(C, align(4))]
#[derive(Clone, Copy, Default, Debug)]
pub struct SkipBlock {
    last_doc: u32,
    blockwand_tf: u32,
    docid_bits: u8,
    tf_bits: u8,
    blockwand_fieldnorm_id: u8,
    reserved: u8,
}

unsafe impl Zeroable for SkipBlock {}
unsafe impl Pod for SkipBlock {}

impl SkipBlock {
    // unfulled block will return invalid block size
    pub fn block_size(&self) -> usize {
        compressed_block_size(self.docid_bits) + compressed_block_size(self.tf_bits)
    }
}