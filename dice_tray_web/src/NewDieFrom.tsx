import React, {useState, useRef, useEffect} from "react";
import { NewDieRequest }  from "./DieDataTypes";

interface NewDieFormProps {
  isOpen: boolean,
  onClose: () => void,
  onSubmitNewDie: (newDieRequest: NewDieRequest) => void;
}

function NewDieForm(props: NewDieFormProps){

  const newDieDialog = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const dialog = newDieDialog.current;
    if (!dialog) return;

    if (props.isOpen){
      dialog.showModal();
    }
    else{
      dialog.close();
    }
  }, [props.isOpen])

  // Initialize state with an object to keep related fields together
  const [formData, setFormData] = useState<NewDieRequest>({
    label: "NewDie",
    sides: 6,
    variance: 25,
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
    props.onSubmitNewDie(formData);
  };

  const closeForm = () => {
    props.onClose();
  }

  return (
    <dialog 
      ref={newDieDialog}
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

        {/* Faces Field */}
        <div>
          <label className="form-label">Faces</label>
          <input
            className="input-field"
            type="number"
            name="sides"
            value={formData.sides}
            onChange={handleChange}
          />
        </div>

        {/* Variance Field */}
        <div>
          <label className="form-label">Variance</label>
          <input
            className="input-field"
            type="number"
            name="variance"
            step="0.1"
            value={formData.variance}
            onChange={handleChange}
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

export const NewDieModal: React.FC<NewDieFormProps> = NewDieForm;