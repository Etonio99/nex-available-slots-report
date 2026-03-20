import { invoke } from '@tauri-apps/api/core';

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

  return { revealFileOrDirectory };
};

export default useFileSystem;
