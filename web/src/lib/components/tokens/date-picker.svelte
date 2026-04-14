<script lang="ts">
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import { cn } from '$lib/utils.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Calendar } from '$lib/components/ui/calendar/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { formatDate } from 'date-fns';
	import { type DateValue, getLocalTimeZone } from '@internationalized/date';

	let { date = $bindable() }: { date: DateValue | undefined } = $props();
</script>

<Popover.Root>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button
				variant="outline"
				class={cn(
					'w-[280px] justify-start text-start font-normal',
					!date && 'text-muted-foreground'
				)}
				{...props}
			>
				<CalendarIcon class="me-2 size-4" />
				{date ? formatDate(date.toDate(getLocalTimeZone()), 'PPP') : 'Select a date'}
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-auto p-0">
		<Calendar bind:value={date} type="single" initialFocus captionLayout="dropdown" />
	</Popover.Content>
</Popover.Root>
