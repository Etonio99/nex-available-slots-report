'use client';

import { createContext, useContext, useRef, useState } from 'react';
import Modal from '../modal';

type ModalOptions = {
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
};

type ModalContextValue = {
  confirm: (options: ModalOptions) => Promise<boolean>;
};

const ModalContext = createContext<ModalContextValue | null>(null);

interface ModalContextProviderProps {
  children: React.ReactNode;
}

type ModalState = ModalOptions & {
  resolve: (value: boolean) => void;
};

export default function ModalContextProvider(props: ModalContextProviderProps) {
  const [modal, setModal] = useState<ModalState | null>(null);
  const resolveRef = useRef<((value: boolean) => void) | null>(null);

  function confirm(options: ModalOptions): Promise<boolean> {
    return new Promise((resolve) => {
      resolveRef.current = resolve;
      setModal({ ...options, resolve });
    });
  }

  function handleConfirm() {
    modal?.resolve(true);
    setModal(null);
  }

  function handleCancel() {
    modal?.resolve(false);
    setModal(null);
  }

  return (
    <ModalContext.Provider value={{ confirm }}>
      {modal && (
        <Modal
          title={modal.title}
          description={modal.description}
          confirmLabel={modal.confirmLabel ?? 'Confirm'}
          cancelLabel={modal.cancelLabel ?? 'Cancel'}
          onConfirm={handleConfirm}
          onCancel={handleCancel}
        />
      )}
      {props.children}
    </ModalContext.Provider>
  );
}

export const useModalContext = () => {
  const context = useContext(ModalContext);
  if (!context) {
    throw new Error(
      'useModalContext must be used within a ModalContextProvider'
    );
  }
  return context;
};
