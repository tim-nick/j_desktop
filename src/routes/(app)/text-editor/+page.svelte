<script lang="ts">
	import { goto } from '$app/navigation';
	import TextEditor from '$lib/components/text-editor/TextEditor.svelte';
	import { onMount } from 'svelte';


	onMount(() => {
		goto('/text-editor/');  // Redirect to the default tab for Text Editor
	});
</script>

<TextEditor />