<script lang="ts">
	import { Shgrid , ServerGridBuilder} from 'shgrid-svelte';
	import 'shgrid-svelte/dist/default-styles.scss';
	import { env } from '$env/dynamic/public';

	let mapper: ServerGridBuilder['mapper'] = (res: any) => {
		return {
			data: res.data,
			count: res.count,
		};
	};
	const url = `${env.PUBLIC_BASE_URL}/contact`;
	let builder = new ServerGridBuilder({
		columns: [
			{ id: 'id', label: 'Id', hidden: true },
			{ id: 'first_name', label: 'First Name' },
			{ id: 'last_name', label: 'Last Name', hidden: true },
			{ id: 'email', label: 'Email' },
			{ id: 'organisation_id', label: 'Organisation' },
			// {
			// 	id: 'organisation',
			// 	label: 'Organisation',
			// 	formatter: (row: Row) =>
			// 		`<p><strong>Name: </strong>${row.organisation.name}</p><p><strong>Postcode: </strong>${row.organisation.postcode}</p>`,
			// 	link: row => `/examples/data/organisation/${(row as Row).organisation.id}`,
			// },
			{ id: 'active', label: 'Active', hidden: true },
			{ id: 'mobile', label: 'Mobile', hidden: true },
			{ id: 'postcode', label: 'Postcode', hidden: true },
		],
		url,
		mapper,
		rowLink: row => `${url}/${row.id}`,
	});
</script>

<section class="hero">
	<div class="left">
		<h2>Shgrid</h2>
		<p>It makes tables</p>
	</div>
	<div class="right">
		<Shgrid {builder} />
	</div>
</section>

<style lang="scss">
	.hero {
		display: flex;
		gap: var(--size-8);
	}
</style>
