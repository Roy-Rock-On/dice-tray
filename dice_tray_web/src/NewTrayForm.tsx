import React, {useState, useRef, useEffect} from "react";
import { NewTrayRequest }  from "./TrayDataTypes";

interface NewTrayFormProps {
  isOpen: boolean,
  onClose: () => void,
  onSubmitNewTray: (newTrayRequest: NewTrayRequest) => void;
}

function NewTrayForm(props: NewTrayFormProps){

  const newTrayDialog = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const dialog = newTrayDialog.current;
    if (!dialog) return;

    if (props.isOpen){
      dialog.showModal();
    }
    else{
      dialog.close();
    }
  }, [props.isOpen])

  // Initialize state with an object to keep related fields together
  const [formData, setFormData] = useState<NewTrayRequest>({
    label: "NewTray",
  });
  
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {

    const { name, value } = e.target;
    
    setFormData((prev) => ({
      ...prev,
      [name]: name === 'label' ? value : Number(value)
    }));
  };

  const handleSubmit = (e: React.SubmitEvent) => {
    e.preventDefault();
    props.onSubmitNewTray(formData);
  };

  const closeForm = () => {
    props.onClose();
  }

  return (
    <dialog 
      ref={newTrayDialog}
      className="form"
      onClose={props.onClose}
    >
      <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
        <div>
          <label className="form-label">Label</label>
          <input
            className="input-field"
            type="text"
            name="label"
            value={formData.label}
            onChange={handleChange}
            placeholder="Enter label name"
          />
        </div>
        <button
          className="button-prime"
          type="submit"
        >
          Submit
        </button>
        <button
          type="button"
          className="button-destructive"
          onClick={closeForm}
        >
          Cancel
        </button>
      </form>
    </dialog>
  );
}

export const NewTrayModal: React.FC<NewTrayFormProps> = NewTrayForm;