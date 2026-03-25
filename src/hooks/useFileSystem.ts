import { invoke } from '@tauri-apps/api/core';
import { documentDir } from '@tauri-apps/api/path';

const useFileSystem = () => {
  const revealFileOrDirectory = async (path: string) => {
    if (!path) {
      return false;
    }

    return invoke<string | undefined>('reveal_file_or_directory', { path })
      .then(() => true)
      .catch((error) => {
        console.error(error ?? 'Undefined error');
        return false;
      });
  };

  const revealDataFolder = async () => {
    const dataPath = (await documentDir()) + '/Nex Analytics';
    return await revealFileOrDirectory(dataPath);
  };

  return { revealFileOrDirectory, revealDataFolder };
};

export default useFileSystem;
