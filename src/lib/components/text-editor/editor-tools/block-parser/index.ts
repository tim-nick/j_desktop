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

    async parse_q_a(content: string) {

      console.log("Flashcard QA check")
      console.log(content)

      try {
        const result = await remark()
          .use(remarkGfm)
          .use(remarkHtml)
          .process(content);
          
        const htmlContent = result.toString();
        console.log("found Flashcard(s): ");
        console.log(htmlContent);
        const [question, answer] = content.split('&gt;&gt;').map(s => s.trim());
        console.log("Question: ", question);
        console.log("Answer:", answer);

        // Return values are good
        return [question, answer];
      } catch (error) {
        console.error('Error parsing QA:', error);
        throw error;  // Re-throw the error if needed
      }

    }

    // debouncing 

    async parsing_flashcards(content:string, api:any) {
      //
      console.log("Flashcard check")
      console.log(content)

      try {
        const result = await remark()
          .use(remarkGfm)
          .use(remarkHtml)
          .process(content);
          
        const htmlContent = result.toString();
        console.log("found Flashcard(s): ");
        console.log(htmlContent);
        const [question, answer] = content.split('&gt;&gt;').map(s => s.trim());
        console.log("Question: ", question);
        console.log("Answer:", answer);


        // Return values are good
        return htmlContent;
      } catch (error) {
        console.error('Error parsing flashcard:', error);
        throw error;  // Re-throw the error if needed
      }
      


      // if (content.includes(" >> ")){
      //   try {
      //     const result = await remark()
      //       .use(remarkGfm)
      //       .use(remarkHtml)
      //       .process(content);
            
      //     const htmlContent = result.toString();
      //     console.log("found Flashcard(s): ")
      //     console.log(htmlContent);
  
      //     // Return values are good
      //     return htmlContent;
      //   } catch (error) {
      //     console.error('Error parsing flashcard:', error);
      //     throw error;  // Re-throw the error if needed
      //   }
      // }

    }

  //   async parse_md_flashcard(content: string) {
  //     if (content.includes('>>')) {
  //         return this.parsing_flashcards_v2(content);
  //     }
  //     // Continue with normal parsing if no flashcard syntax is detected
  //     // ...
  // }

  // async parsing_flashcards_v2(content: string) {
  //     const [question, answer] = content.split('>>').map(s => s.trim());
  //     return {
  //         question,
  //         answer,
  //         type: 'flashcard'
  //     };
  // }

    //&nbsp; detecter for headings and lists 

    // changeSomeDataProperty(data:any, api:api) {
    //     this.data['some-property'] = 'some-value'
    
    //     // Tell Editor to know that block was changed
    //     this.blockAPI.dispatchChange()
    // }

}



// transformListItemToFlashcard(itemContent, listItem) {
//   // Split the current content into question and answer
//   const [question, answer] = itemContent.innerText.split('>>').map(s => s.trim());

//   // Create a new flashcard block in place of the list item
//   this.insertFlashcardBlock(question, answer, listItem);

//   // Optionally remove the current list item after creating the flashcard block
//   listItem.remove();
// }

// insertFlashcardBlock(question, answer, listItem) {
//   const currentIndex = this.api.blocks.getCurrentBlockIndex();

//   // Insert the flashcard block at the current index
//   this.api.blocks.insert('flashcard', {
//     question: question || 'Enter your question...',
//     answer: answer || 'Enter your answer...'
//   }, {}, currentIndex + 1, true);

//   const newBlock = this.api.blocks.getBlockByIndex(currentIndex + 1);
//   if (newBlock) {
//     const answerElem = newBlock.holder.querySelector('.flashcard-answer');
//     if (answerElem) {
//       this.focusItem(answerElem);
//     }
//   }
// }

