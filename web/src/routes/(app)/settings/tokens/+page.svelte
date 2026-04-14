<script lang="ts">
	import DataTable from '$lib/components/tokens/data-table.svelte';
	import { Button } from '$lib/components/ui/button';

	import KeyRound from '@lucide/svelte/icons/key-round';

	let { data } = $props();
</script>

<div class="p-4">
	<div class="flex justify-between">
		<div class="">
			<h1 class="text-lg font-medium md:text-xl">Tokens List</h1>
			<p class="text-sm">Manage your API tokens</p>
		</div>
		<Button>
			<KeyRound /> Generate New Token
		</Button>
	</div>
	<DataTable data={data.tokens} />
</div>
