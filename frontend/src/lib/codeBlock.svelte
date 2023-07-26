<script lang="ts">
    import '../styles/prism-themes/one-dark.scss'

	import Prism from 'prismjs';
	import 'prism-svelte';

	export let language: string;
	export let code: string;

	$: html = Prism.highlight(code, Prism.languages[language], language);
	async function copy() {
		await navigator.clipboard.writeText(code);
		copyText = 'copied';
		setTimeout(() => (copyText = 'copy'), 2000);
	}
	let copyText = 'copy';
</script>

<div class="wrapper">
	<pre>
        <code>{@html html}</code>
    </pre>
	<button class="btn-primary" on:click={copy}>{copyText}</button>
</div>

<style lang="scss">
	.wrapper {
		position: relative;
		width: max-content;
		button {
			position: absolute;
			top: size(4);
			right: size(4);
		}
	}
	pre {
		padding: size(4);
		border-radius: size(2);
		white-space: pre-wrap;
		background-color: #333;
		color: #ccc;
	}
</style>
