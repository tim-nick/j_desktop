<script>
    // You can add any necessary logic here, such as handling input events.
    let content = "";  // This will hold the content of the editor if needed
  </script>
  
  <div
    contenteditable="true"
    bind:innerHTML={content}
    class="editor"
    on:input={(e) => content = e.target.innerHTML}
  ></div>
  
  <style>
    /* The editor will fill its parent container */
    .editor {
      width: 100%;
      height: 100%;
      background-color: white;
      color: black;
      padding: 10px;
      border-radius: 5px;
      box-sizing: border-box;  /* Ensure padding doesn't affect width/height */
      overflow-y: auto;  /* Allow scrolling if content overflows */
    }
  
    /* Optional: Add some basic styling for paragraphs */
    .editor p {
      margin: 0;
      padding: 5px 0;
    }
  </style>