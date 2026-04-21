<script lang="ts">
	import { cn, type WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="empty"
	class={cn(
		'min-w-0 gap-4 rounded-lg p-12 flex w-full flex-1 flex-col items-center justify-center border-dashed text-center text-balance',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
