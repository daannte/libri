<script lang="ts">
	import DataTable from '$lib/components/tokens/data-table.svelte';

	let { data } = $props();
</script>

<DataTable data={data.tokens} />
