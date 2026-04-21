<script>
	import { resolve } from '$app/paths';
	import { Select } from '@shiori/components';
	import { resetMode, setMode, userPrefersMode } from 'mode-watcher';
</script>

<div>Settings</div>

<a href={resolve('/(app)/settings/tokens')}>tokens</a>

<Select.Root type="single">
	<Select.Trigger class="w-[180px]">
		{userPrefersMode.current.charAt(0).toUpperCase() + userPrefersMode.current.slice(1)}
	</Select.Trigger>
	<Select.Content>
		<Select.Item value="light" onclick={() => setMode('light')}>Light</Select.Item>
		<Select.Item value="dark" onclick={() => setMode('dark')}>Dark</Select.Item>
		<Select.Item value="system" onclick={() => resetMode()}>System</Select.Item>
	</Select.Content>
</Select.Root>
