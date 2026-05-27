import React, {useState} from "react";

function NewDieForm({appHandle}){
  // Initialize state with an object to keep related fields together
  const [formData, setFormData] = useState({
    label: '',
    count: 0,
    variance: 0,
    color: '#3b82f6' // Default blue
  });

  // Generic change handler for all inputs
  const handleChange = (e) => {
    const { name, value, type } = e.target;
    
    setFormData((prev) => ({
      ...prev,
      // Convert to number if the input type is number
      [name]: type === 'number' ? parseFloat(value) : value,
    }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    console.log('Form Submitted:', formData);
    alert(`Submitted: ${formData.label}`);
  };

  return (
    <div class="form">
      <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
        <div>
          <label class="form-label">Label</label>
          <input
            class="input-field"
            type="text"
            name="label"
            value={formData.label}
            onChange={handleChange}
            placeholder="Enter label name"
          />
        </div>

        {/* Faces Field */}
        <div>
          <label class="form-label">Faces</label>
          <input
            class="input-field"
            type="number"
            name="Faces"
            value={formData.Faces}
            onChange={handleChange}
          />
        </div>

        {/* Variance Field */}
        <div>
          <label class="form-label">Variance</label>
          <input
            class="input-field"
            type="number"
            name="variance"
            step="0.1"
            value={formData.variance}
            onChange={handleChange}
          />
        </div>

        {/* Color Field */}
        <div>
          <label class="form-label">Color</label>
          <input
            class="input-color"
            type="color"
            name="color"
            value={formData.color}
            onChange={handleChange}
          />
        </div>

        <button
          class="button-prime"
          type="submit"
        >
          Submit Data
        </button>
      </form>
    </div>
  );
}

export default NewDieForm;