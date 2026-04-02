<script lang="ts">
	import { goto } from '$app/navigation';
	import { createClient } from '@shiori/api-client';

	import * as Dialog from './ui/dialog';
	import { Button } from './ui/button';

	interface Props {
		id: number;
		isOpen: boolean;
	}

	let client = createClient({ fetch });

	let { id, isOpen = $bindable() }: Props = $props();

	let error = $state('');

	async function handleDelete() {
		error = '';

		try {
			let res = await client.DELETE('/api/v1/media/{id}', { params: { path: { id } } });
			if (res.error || res.data) throw new Error('Failed to delete media');
			goto('/');
		} catch (e) {
			console.error('Failed delete: ', e);
			error = 'Failed to delete media';
		} finally {
			isOpen = false;
		}
	}
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Confirm Delete</Dialog.Title>
			<Dialog.Description>Do you really want to delete this?</Dialog.Description>
		</Dialog.Header>
		<div class="flex gap-4">
			<Button onclick={() => (isOpen = false)}>Cancel</Button>
			<Button onclick={handleDelete} variant="destructive">Delete</Button>
		</div>
		<span class="text-sm text-destructive">{error}</span>
	</Dialog.Content>
</Dialog.Root>
