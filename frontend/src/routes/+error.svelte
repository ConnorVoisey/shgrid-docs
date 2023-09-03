<script lang="ts">
	import { page } from '$app/stores';
	import '$styles/main.scss';
	import '$styles/reset.scss';
</script>

<div class="content">
	<h1 class="title">Error: {$page.status}</h1>
	<h2 class="subtitle">{$page?.error?.message}</h2>
	<a href="/" class="link">Back to home &rarr;</a>
</div>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
	}
	.link {
		margin-top: size(4);
	}
</style>
