<script lang="ts">
	import CreateDialog from '$lib/components/tokens/create-dialog.svelte';
	import DataTable from '$lib/components/tokens/data-table.svelte';

	let { data } = $props();
</script>

<div class="p-4">
	<div class="flex justify-between">
		<div class="">
			<h1 class="text-lg font-medium md:text-xl">Tokens List</h1>
			<p class="text-sm">Manage your API tokens</p>
		</div>
		<CreateDialog />
	</div>
	<DataTable data={data.tokens} />
</div>
