// import { onMount } from 'svelte';
import { remark } from 'remark';
import remarkGfm from 'remark-gfm';
import remarkHtml from 'remark-html';

export class BlockParser {
  

    async parse_md(content: string) {
        console.log("Parser triggered")
        try {
          const result = await remark()
            .use(remarkGfm)
            .use(remarkHtml)
            .process(content);
            
          const htmlContent = result.toString();
          console.log("found: ")
          console.log(htmlContent);
          // REturn values are good
          return htmlContent;
        } catch (error) {
          console.error('Error parsing markdown:', error);
          throw error;  // Re-throw the error if needed
        }
    }

    // debouncing 

    //&nbsp; detecter for headings and lists 

    // changeSomeDataProperty(data:any, api:api) {
    //     this.data['some-property'] = 'some-value'
    
    //     // Tell Editor to know that block was changed
    //     this.blockAPI.dispatchChange()
    // }

}


