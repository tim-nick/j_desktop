/**
 * Build styles
 */
import './index.css';

import { IconText } from '@codexteam/icons';
import makeFragment from './utils/makeFragment';
import {BlockParser} from '../../block-parser/index'

import type {
  API,
  ConversionConfig,
  HTMLPasteEvent,
  PasteConfig,
  SanitizerConfig,
  ToolConfig,
  ToolboxConfig,
} from '@editorjs/editorjs';
// My import
import NestedList, { ListData } from '@editorjs/nested-list'; // Adjust the import path as needed
// import Flashcard from '$lib/components/text-editor/editor-tools/Flashcards/';
import Flashcard, {FlashcardData} from "../../Flashcards"

/**
 * Base Paragraph Block for the Editor.js.
 * Represents a regular text block
 *
 * @author CodeX (team@codex.so)
 * @copyright CodeX 2018
 * @license The MIT License (MIT)
 */

/**
 * @typedef {object} ParagraphConfig
 * @property {string} placeholder - placeholder for the empty paragraph
 * @property {boolean} preserveBlank - Whether or not to keep blank paragraphs when saving editor data
 */
export interface ParagraphConfig extends ToolConfig {
  /**
   * Placeholder for the empty paragraph
   */
  placeholder?: string;

  /**
   * Whether or not to keep blank paragraphs when saving editor data
   */
  preserveBlank?: boolean;
}

/**
 * @typedef {object} ParagraphData
 * @description Tool's input and output data format
 * @property {string} text — Paragraph's content. Can include HTML tags: <a><b><i>
 */
export interface ParagraphData {
  /**
   * Paragraph's content
   */
  text: string;
}

/**
 * @typedef {object} ParagraphParams
 * @description Constructor params for the Paragraph tool, use to pass initial data and settings
 * @property {ParagraphData} data - Preload data for the paragraph.
 * @property {ParagraphConfig} config - The configuration for the paragraph.
 * @property {API} api - The Editor.js API.
 * @property {boolean} readOnly - Is paragraph is read-only.
 */
interface ParagraphParams {
  /**
   * Initial data for the paragraph
   */
  data: ParagraphData;
  /**
   * Paragraph tool configuration
   */
  config: ParagraphConfig;
  /**
   * Editor.js API
   */
  api: API;
  /**
   * Is paragraph read-only.
   */
  readOnly: boolean;
}

/**
 * @typedef {object} ParagraphCSS
 * @description CSS classes names
 * @property {string} block - Editor.js CSS Class for block
 * @property {string} wrapper - Paragraph CSS Class
 */
interface ParagraphCSS {
  /**
   * Editor.js CSS Class for block
   */
  block: string;
  /**
   * Paragraph CSS Class
   */
  wrapper: string;
}

export default class Paragraph {
  /**
   * Default placeholder for Paragraph Tool
   *
   * @returns {string}
   * @class
   */
  static get DEFAULT_PLACEHOLDER() {
    return '';
  }

  /**
   * The Editor.js API
   */
  api: API;

  /**
   * Is Paragraph Tool read-only
   */
  readOnly: boolean;

  /**
   * Paragraph Tool's CSS classes
   */
  private _CSS: ParagraphCSS;

  /**
   * Placeholder for Paragraph Tool
   */
  private _placeholder: string;

  /**
   * Paragraph's data
   */
  private _data: ParagraphData;

  /**
   * Paragraph's main Element
   */
  private _element: HTMLDivElement | null;

  /**
   * Whether or not to keep blank paragraphs when saving editor data
   */
  private _preserveBlank: boolean;

  /**
   * Render plugin`s main Element and fill it with saved data
   *
   * @param {object} params - constructor params
   * @param {ParagraphData} params.data - previously saved data
   * @param {ParagraphConfig} params.config - user config for Tool
   * @param {object} params.api - editor.js api
   * @param {boolean} readOnly - read only mode flag
   */
  constructor({ data, config, api, readOnly }: ParagraphParams) {
    this.api = api;
    this.readOnly = readOnly;

    this._CSS = {
      block: this.api.styles.block,
      wrapper: 'ce-paragraph',
    };

    if (!this.readOnly) {
      this.onKeyUp = this.onKeyUp.bind(this);
    }

    /**
     * Placeholder for paragraph if it is first Block
     *
     * @type {string}
     */
    this._placeholder = config.placeholder
      ? config.placeholder
      : Paragraph.DEFAULT_PLACEHOLDER;
    this._data = data ?? {};
    this._element = null;
    this._preserveBlank = config.preserveBlank ?? false;
  }

  /**
   * Check if text content is empty and set empty string to inner html.
   * We need this because some browsers (e.g. Safari) insert <br> into empty contenteditanle elements
   *
   * @param {KeyboardEvent} e - key up event
   */
  async onKeyUp(e: KeyboardEvent): void {
    // if (e.code !== 'Backspace' && e.code !== 'Delete') {
    //   return;
    // }
    
    console.log(e)
    console.log(this._element?.innerHTML)
    const parser = new BlockParser();
    let test = this._element?.innerHTML;

    // Debounce test var 
    
    let lastChar = '';
    if (test != null) {
      if (test.match("&nbsp;")) {
        lastChar = '&nbsp;';
        console.log(lastChar);
        // Now remove &nbsp; and check for html elements
        test = test.replace("&nbsp;", "");
        console.log("Test: " + test);
        const parsedHTML:string  = await parser.parse_md(test);
        const parsedFlashcard:string = await parser.parsing_flashcards(test, this.api);
        console.log("Found: "+ parsedHTML)
        
        // Determine the level dynamically based on the tag
        const match_heading = parsedHTML.match(/<h([1-6])>(.*?)<\/h[1-6]>/);
        // const match_list = parsedHTML.match(/^- (.*?)$/gm);
        console.log("match list:")
        // const match_list = parsedHTML.match(/<ul>\s*<li>(.*?)<\/li>\s*<\/ul>/gs);

        // if (match_list) {
        //   match_list.forEach((match, index) => {
        //     console.log(`Match ${index + 1}:`, match);
        //   });
        // } else {
        //   console.log("No matches found.");
        // }

        const match_list = parsedHTML.match(/<li>(.*?)<\/li>/);

        if (match_list) {
          match_list.forEach((match, index) => {
            console.log(`Match ${index + 1}:`, match);
          });
        } else {
          console.log("No matches found.");
        }


        // Heading
        let level;
        let text;

        //list
        let nestedListData:ListData;

        //flashcard 
        let flashcardData:FlashcardData;
        

        if (match_heading) {
          level = parseInt(match_heading[1], 10); // Extracts the level (1-6)
          text = match_heading[2]; // Extracts the inner text

        } 
        if (match_list) {
          console.log("Found a list ")
          console.log(test)
          test = ""
          // test.replace("-", "");
          // test.replace(" ", "");
          // currentIndex = this.api.blocks.getCurrentBlockIndex();
          // Define the data for the Nested List block
          nestedListData = {
            style: 'unordered', // or 'ordered'
            items: [
              {
                content: test,
                items: [] // No nested items
              }
            ]
          };
        }

        if (test != "") {
          console.log(e.key, e.altKey)
        }


        const index = this.api.blocks.getCurrentBlockIndex();
        // console.log("Current Block result : " + String(level) + " " + text);


        // Debug
        console.log("############################")
        console.log('')
        console.log("current Block index: ")
        console.log(index)

        console.log('')

        console.log("match_list: ")
        console.log(match_list)

        console.log('')

        console.log("match_heading: ")
        console.log(match_heading)

        

        if (level != null && lastChar != null && lastChar == '&nbsp;') {
          console.log("Outer insertion")


            if (match_heading) {

              // Insert the header block based on the parsed data
              this.api.blocks.insert("header", {
                text: text,
                level: level, // Dynamically set the level (h1-h6)
              }, {}, index, true);
              console.log("Inserted block: " + String(level) + " " + text);
              this.api.blocks.delete(index+1);

            }
            

          // Insert the header block based on the parsed data
          // this.api.blocks.insert("header", {
          //   text: text,
          //   level: level, // Dynamically set the level (h1-h6)
          // }, {}, index, true);
          // console.log("Inserted block: " + String(level) + " " + text);
          // this.api.blocks.delete(index+1);
          
        } else {
          console.log("wtf ");
        }

        if (match_list) {

          
          // Insert the Nested List block at the current index
          this.api.blocks.insert('nestedList', nestedListData, {}, index, true);
          this.api.blocks.delete(index+1); 
          console.log("Inserted:")
          console.log(nestedListData)
        } else {
          console.log("NOthing to enter");
        }
      }
    }
    

    // Check mapping 

    

    

    if (!this._element) {
      return;
    }

    const { textContent } = this._element;

    if (textContent === '' && test !=  null) {
      this._element.innerHTML = '';
    }



    const totalBlocks = this.api.blocks.getBlocksCount();
    const blockIds: string[] = [];
  
    for (let i = 0; i < totalBlocks; i++) {
      const block = this.api.blocks.getBlockByIndex(i);
      if (block) {
        blockIds.push(block.name);
      }
    }
  
     console.log(blockIds)
  }

  /**
   * Create Tool's view
   *
   * @returns {HTMLDivElement}
   * @private
   */
  drawView(): HTMLParagraphElement {
    console.log("user typed enter ")
    console.log(this._data.text)
    const p = document.createElement('P');  // Creates a <p> element
    p.classList.add(this._CSS.wrapper, this._CSS.block);  // Adds the same classes
    p.contentEditable = 'false';  // Makes the paragraph non-editable initially
    p.dataset.placeholderActive = this.api.i18n.t(this._placeholder);
  
    if (this._data.text) {
      p.innerHTML = this._data.text;  // Inserts the saved text into the <p>
    }
    console.log(p.innerHTML.length)
  
    if (!this.readOnly) {
      p.contentEditable = 'true';  // Allows content editing if not read-only
      p.addEventListener('keyup', this.onKeyUp);
      console.log("Added event listener to paragraph")
    }
  
    return p;  // Returns the <p> element
  }

  /**
   * Return Tool's view
   *
   * @returns {HTMLDivElement}
   */
  render(): HTMLDivElement {
    this._element = this.drawView();
    console.log("log")
    return this._element;
  }

  /**
   * Method that specified how to merge two Text blocks.
   * Called by Editor.js by backspace at the beginning of the Block
   *
   * @param {ParagraphData} data
   * @public
   */
  merge(data: ParagraphData): void {
    if (!this._element) {
      return;
    }

    this._data.text += data.text;

    /**
     * We use appendChild instead of innerHTML to keep the links of the existing nodes
     * (for example, shadow caret)
     */
    const fragment = makeFragment(data.text);

    this._element.appendChild(fragment);

    this._element.normalize();
  }

  /**
   * Validate Paragraph block data:
   * - check for emptiness
   *
   * @param {ParagraphData} savedData — data received after saving
   * @returns {boolean} false if saved data is not correct, otherwise true
   * @public
   */
  validate(savedData: ParagraphData): boolean {
    if (savedData.text.trim() === '' && !this._preserveBlank) {
      return false;
    }

    return true;
  }

  /**
   * Extract Tool's data from the view
   *
   * @param {HTMLDivElement} toolsContent - Paragraph tools rendered view
   * @returns {ParagraphData} - saved data
   * @public
   */
  save(toolsContent: HTMLDivElement): ParagraphData {
    console.log(toolsContent.innerHTML)
    console.log("Paragraph saved")
    return {
      text: toolsContent.innerHTML,
    };
  }

  /**
   * On paste callback fired from Editor.
   *
   * @param {HTMLPasteEvent} event - event with pasted data
   */
  onPaste(event: HTMLPasteEvent): void {
    const data = {
      text: event.detail.data.innerHTML,
    };
    // TODO implement markdown parsing here also ? 

    this._data = data;
    console.log(this._data)

    /**
     * We use requestAnimationFrame for performance purposes
     */
    window.requestAnimationFrame(() => {
      if (!this._element) {
        return;
      }
      this._element.innerHTML = this._data.text || '';
    });
  }

  /**
   * Enable Conversion Toolbar. Paragraph can be converted to/from other tools
   * @returns {ConversionConfig}
   */
  static get conversionConfig(): ConversionConfig {
    return {
      export: 'text', // to convert Paragraph to other block, use 'text' property of saved data
      import: 'text', // to covert other block's exported string to Paragraph, fill 'text' property of tool data
    };
  }

  /**
   * Sanitizer rules
   * @returns {SanitizerConfig} - Edtior.js sanitizer config
   */
  static get sanitize(): SanitizerConfig {
    return {
      text: {
        br: true,
      },
    };
  }

  /**
   * Returns true to notify the core that read-only mode is supported
   *
   * @returns {boolean}
   */
  static get isReadOnlySupported(): boolean {
    return true;
  }

  /**
   * Used by Editor paste handling API.
   * Provides configuration to handle P tags.
   *
   * @returns {PasteConfig} - Paragraph Paste Setting
   */
  static get pasteConfig(): PasteConfig {
    return {
      tags: ['P'],
    };
  }

  /**
   * Icon and title for displaying at the Toolbox
   *
   * @returns {ToolboxConfig} - Paragraph Toolbox Setting
   */
  static get toolbox(): ToolboxConfig {
    return {
      icon: IconText,
      title: 'Text',
    };
  }
}
