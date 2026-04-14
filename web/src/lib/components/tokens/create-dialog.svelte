<script lang="ts">
	import { createClient } from '@shiori/api-client';

	import * as Dialog from '../ui/dialog';
	import { Label } from '../ui/label';
	import { Input } from '../ui/input';
	import { Button, buttonVariants } from '../ui/button';
	import DatePicker from './date-picker.svelte';
	import { type DateValue, getLocalTimeZone } from '@internationalized/date';

	import Plus from '@lucide/svelte/icons/plus';
	import { endOfDay } from 'date-fns';

	let client = createClient({ fetch });

	let isOpen = $state(false);
	let name = $state('');
	let date = $state<DateValue>();

	async function handleToken() {
		if (name.length === 0) return;

		try {
			const res = await client.POST('/api/v1/tokens', {
				body: { expires_at: date?.toDate(getLocalTimeZone()).toISOString() ?? null, name }
			});

			if (!res.data || res.error) throw new Error();
		} catch {
			console.error('Failed to generate token');
		}
	}
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Trigger type="button" class={buttonVariants({ variant: 'default' })}>
		<Plus /> Generate New Token
	</Dialog.Trigger>
	<Dialog.Content showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Generate Token</Dialog.Title>
			<Dialog.Description
				>Please provide the necessary details below to generate a new token.</Dialog.Description
			>
		</Dialog.Header>
		<div class="flex flex-col gap-4">
			<div class="grid gap-2">
				<Label for="name">Name</Label>
				<Input id="name" name="name" placeholder="Koreader Sync" bind:value={name} />
			</div>
			<DatePicker bind:date />
		</div>
		<Dialog.Footer>
			<Dialog.Close
				onclick={() => (isOpen = false)}
				class={buttonVariants({ variant: 'secondary' })}>Cancel</Dialog.Close
			>
			<Button onclick={handleToken}>Generate</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
