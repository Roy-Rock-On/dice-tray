import React, { useState } from "react";

function DiceBag({appHandle}){
    const [diceBag, setDiceBag] = useState(null);

    return (
        <div className="tray">
            <DieForm/>
        </div>
    )
}

function DieForm() {
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
    <div style={{ maxWidth: '400px', margin: '20px auto', fontFamily: 'sans-serif' }}>
      <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>Label</label>
          <input
            type="text"
            name="label"
            value={formData.label}
            onChange={handleChange}
            placeholder="Enter label name"
            style={{ width: '100%', padding: '8px' }}
          />
        </div>

        {/* Count Field */}
        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>Count</label>
          <input
            type="number"
            name="count"
            value={formData.count}
            onChange={handleChange}
            style={{ width: '100%', padding: '8px' }}
          />
        </div>

        {/* Variance Field */}
        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>Variance</label>
          <input
            type="number"
            name="variance"
            step="0.1"
            value={formData.variance}
            onChange={handleChange}
            style={{ width: '100%', padding: '8px' }}
          />
        </div>

        {/* Color Field */}
        <div>
          <label style={{ display: 'block', marginBottom: '5px' }}>Color</label>
          <input
            type="color"
            name="color"
            value={formData.color}
            onChange={handleChange}
            style={{ width: '100%', height: '40px', cursor: 'pointer' }}
          />
        </div>

        <button 
          type="submit" 
          style={{ 
            padding: '10px', 
            backgroundColor: '#007bff', 
            color: 'white', 
            border: 'none', 
            borderRadius: '4px',
            cursor: 'pointer' 
          }}
        >
          Submit Data
        </button>
      </form>
    </div>
  );
}

export default DiceBag;