use bisharper_bank::{BankArchive, BankFileMeta};
use rfsa::{PathLike, VFileSystem, VPath};
use crate::{GameFile, GameFileSystem, GameMeta};

impl Into<GameMeta> for BankFileMeta {
    fn into(self) -> GameMeta {
        GameMeta {

        }
    }
}

impl GameFileSystem {
    pub fn bank_import_file(&mut self, archive: &BankArchive, path: &VPath) -> rfsa::Result<Option<GameFile>> {
        self.bank_import_file_advanced(archive, path, None)
    }

    pub fn bank_import_file_advanced(&mut self, archive: &BankArchive, path: &VPath, overwrite_prefix: Option<&VPath>) -> rfsa::Result<Option<GameFile>> {
        let contents = archive.fs_contents(&path)?;
        let metadata: GameMeta = archive.fs_meta(&path)?.into();
        let file = GameFile::create(metadata, contents);
        let root = overwrite_prefix.unwrap_or(&archive.fs_root());

        self.fs_insert(&root.join(path), file)
    }

    pub fn bank_import_archive_advanced(&mut self, archive: &BankArchive, overwrite_prefix: Option<&VPath>) -> rfsa::Result<Vec<(VPath, GameFile)>> {
        let mut replaced = vec![];

        let root = overwrite_prefix.unwrap_or(&archive.fs_root());

        for bank_path in archive.fs_iter()? {
            match self.bank_import_file_advanced(archive, &bank_path, Some(root))? {
                None => continue,
                Some(removed) => replaced.push((bank_path, removed))
            }
        }

        Ok(replaced)
    }

}