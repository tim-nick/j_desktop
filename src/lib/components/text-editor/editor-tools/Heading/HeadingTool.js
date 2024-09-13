class HeadingTool {
    static get toolbox() {
      return {
        title: 'Heading',
        icon: '<svg width="17" height="17" viewBox="0 0 1024 1024"><path d="M512 0C229.2 0 0 229.2 0 512c0 282.8 229.2 512 512 512s512-229.2 512-512C1024 229.2 794.8 0 512 0zM512 896C282.8 896 128 741.2 128 512S282.8 128 512 128s384 154.8 384 384S741.2 896 512 896zM736 432H576v-32h160v-64H576V224h-64v112H320v-96h-64v96H96v64h160v96H96v64h160v160h64V608h192v96h64v-96h160v-64H576v-96h160v-64z"/></svg>',
      };
    }
  
    constructor({data, config}) {
      this.data = {
        text: data.text || '',
        level: data.level || 1,
      };
      this.config = config || {};
      this.wrapper = null;
    }
  
    render() {
      this.wrapper = document.createElement('div');
      
      // Create heading selector (h1-h6)
      const select = document.createElement('select');
      select.innerHTML = `
        <option value="1">H1</option>
        <option value="2">H2</option>
        <option value="3">H3</option>
        <option value="4">H4</option>
        <option value="5">H5</option>
        <option value="6">H6</option>
      `;
      select.value = this.data.level;
  
      // Heading input field
      const input = document.createElement('input');
      input.type = 'text';
      input.placeholder = 'Enter heading text...';
      input.value = this.data.text;
  
      // Set up event listener to update the heading level
      select.addEventListener('change', () => {
        this.data.level = select.value;
        this._updateHeading(input.value);
      });
  
      // Add input event listener
      input.addEventListener('input', (event) => {
        this.data.text = event.target.value;
        this._updateHeading(this.data.text);
      });
  
      this.wrapper.appendChild(select);
      this.wrapper.appendChild(input);
  
      // Render heading on initialization if data exists
      if (this.data.text) {
        this._updateHeading(this.data.text);
      }
  
      return this.wrapper;
    }
  
    _updateHeading(text) {
      // Remove existing heading if present
      const existingHeading = this.wrapper.querySelector('h1, h2, h3, h4, h5, h6');
      if (existingHeading) {
        this.wrapper.removeChild(existingHeading);
      }
  
      // Create new heading based on the selected level
      const heading = document.createElement(`h${this.data.level}`);
      heading.textContent = text;
  
      this.wrapper.appendChild(heading);
    }
  
    save(blockContent) {
      const input = blockContent.querySelector('input');
      return {
        text: input.value,
        level: this.data.level,
      };
    }
  
    validate(savedData) {
      return savedData.text.trim() !== '';
    }
  }
  
  export default HeadingTool;