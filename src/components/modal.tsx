import Button from './button';

interface ModalProps {
  title: string;
  description: string;
  confirmLabel: string;
  cancelLabel: string;
  onConfirm: () => void;
  onCancel: () => void;
}

const Modal = (props: ModalProps) => {
  return (
    <div className="absolute inset-0 z-200 flex items-center justify-center bg-sandstone-950/50">
      <div className="bg-sandstone-50 rounded-md shadow-lg shadow-sandstone-950/30 p-6 w-96 flex flex-col gap-4">
        <div>
          <h2 className="text-sandstone-900 font-bold text-lg">
            {props.title}
          </h2>
          <p className="text-sandstone-700 text-sm mt-1">{props.description}</p>
        </div>
        <div className="flex justify-end gap-2">
          <Button
            label={props.cancelLabel}
            onClick={props.onCancel}
            style="tertiary"
          />
          <Button
            label={props.confirmLabel}
            onClick={props.onConfirm}
            style="primary"
          />
        </div>
      </div>
    </div>
  );
};

export default Modal;
