<script lang="ts">
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { createClient } from '@shiori/api-client';
	import { invalidate } from '$app/navigation';

	const client = createClient({ fetch });

	let { key_id }: { key_id: string } = $props();

	async function handleDelete() {
		try {
			const res = await client.DELETE('/api/v1/tokens/{key_id}', { params: { path: { key_id } } });
			if (res.error) throw new Error();
			invalidate('tokens:load');
		} catch {
			console.error('Failed to delete token');
		}
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon">
				<EllipsisIcon />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Item variant="destructive" onclick={handleDelete}>
				<span>Delete</span>
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
