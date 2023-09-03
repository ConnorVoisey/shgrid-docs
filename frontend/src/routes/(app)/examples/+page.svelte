<script lang="ts">
	type Example = { label: string; linkSegment: string; description: string; tags: string[] };
	const serverSideExamples: Example[] = [
		{
			label: 'Basic Example',
			linkSegment: 'contacts',
			description: 'Fake contacts module to display basic usage',
			tags: ['columns', 'sort', 'filter'],
		},
		{
			label: 'Sorting',
			linkSegment: 'sorting',
			description: 'Predefined sorting and disabling sorting for columns',
			tags: ['sort', 'columns'],
		},
		{
			label: 'Multi Sort',
			linkSegment: 'multi-layer-sort',
			description: 'Multi layer sorting',
			tags: ['sort', 'columns'],
		},
		{
			label: 'Filtering',
			linkSegment: 'filter',
			description: 'Predefined filters and disabling filters for columns',
			tags: ['filter', 'columns'],
		},
	];
</script>

<svelte:head>
	<title>Examples</title>
</svelte:head>
<div class="wrapper">
	<h3 class="title">Server side examples</h3>
	<ul class="cards">
		{#each serverSideExamples as example}
			<li>
				<a href={`/examples/${example.linkSegment}`} class="card-underline-hover">
					<h4 class="subtitle">{example.label}</h4>
					<p>{example.description}</p>
					<ul class="tags">
						{#each example.tags as tag}
							<li class="tag">{tag}</li>
						{/each}
					</ul>
				</a>
			</li>
		{/each}
	</ul>
</div>

<style lang="scss">
	.wrapper {
		@include content-width;
		padding-top: size(16);
	}

	.cards {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: size(8);
	}
</style>
