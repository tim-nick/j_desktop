<script lang="ts">
	import { goto } from '$app/navigation';
	import EditorJsEditor from '$lib/components/text-editor/EditorJsEditor.svelte';
	import TextEditor from '$lib/components/text-editor/TextEditor.svelte';
	import { onMount } from 'svelte';


	onMount(() => {
		goto('/text-editor/');  // Redirect to the default tab for Text Editor
	});
</script>

<EditorJsEditor />