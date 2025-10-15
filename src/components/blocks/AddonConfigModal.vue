<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { ExternalLink } from 'lucide-vue-next';
import { openPath } from '@tauri-apps/plugin-opener';
import { marked } from 'marked';
import { TauriAPI } from '@/services/tauri';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Label } from '@/components/ui/label';
import type { AddonStatus, UpdateMode } from '@/types';

interface Props {
  open: boolean;
  addon: AddonStatus | null;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  'update:open': [value: boolean];
  'configSaved': [];
}>();

const readme = ref<string>('');
const isLoadingReadme = ref(false);

// Update configuration
const updateMode = ref<UpdateMode>('release');
const selectedBranch = ref<string>('main');
const availableBranches = ref<string[]>([]);
const isLoadingBranches = ref(false);
const isSaving = ref(false);
const hasUnsavedChanges = ref(false);

const statusBadgeClass = computed(() => {
  if (!props.addon) return 'bg-gray-500';
  if (props.addon.status === 'up-to-date') return 'bg-green-400';
  if (props.addon.status === 'update-available') return 'bg-orange-400';
  if (!props.addon.is_installed) return 'bg-gray-500';
  return 'bg-gray-500';
});

const statusText = computed(() => {
  if (!props.addon) return 'Unknown';
  if (props.addon.status === 'up-to-date') return 'Up to date';
  if (props.addon.status === 'update-available') return 'Update available';
  if (props.addon.status === 'checking') return 'Checking...';
  if (!props.addon.is_installed) return 'Not installed';
  return 'Unknown';
});

const openGithubRepo = async () => {
  if (!props.addon) return;
  const url = `https://github.com/${props.addon.definition.github_owner}/${props.addon.definition.github_repo}`;
  await openPath(url);
};

const readmeHtml = computed(() => {
  if (!readme.value) return '<p class="text-muted-foreground text-sm">No README available</p>';
  return marked(readme.value);
});

// Load branches from GitHub
const loadBranches = async () => {
  if (!props.addon) return;

  isLoadingBranches.value = true;
  try {
    availableBranches.value = await TauriAPI.fetchGithubBranches(
      props.addon.definition.github_owner,
      props.addon.definition.github_repo
    );
  } catch (error) {
    console.error('Failed to load branches:', error);
    availableBranches.value = ['main', 'master']; // Fallback
  } finally {
    isLoadingBranches.value = false;
  }
};

// Initialize configuration when addon changes
watch(() => props.addon, (newAddon) => {
  if (!newAddon) return;

  // Set initial values from addon definition
  updateMode.value = newAddon.definition.update_mode;
  selectedBranch.value = newAddon.definition.branch || 'main';

  // Load branches if in branch mode
  if (updateMode.value === 'branch') {
    loadBranches();
  }
}, { immediate: true });

// Load branches when switching to branch mode
watch(updateMode, (newMode) => {
  if (newMode === 'branch' && availableBranches.value.length === 0) {
    loadBranches();
  }
  hasUnsavedChanges.value = true;
});

// Track changes to branch selection
watch(selectedBranch, () => {
  hasUnsavedChanges.value = true;
});

// Save configuration
const saveConfiguration = async () => {
  if (!props.addon) return;

  isSaving.value = true;
  try {
    await TauriAPI.saveAddonOverride(
      props.addon.definition.local_name,
      updateMode.value,
      updateMode.value === 'branch' ? selectedBranch.value : undefined
    );

    hasUnsavedChanges.value = false;
    emit('configSaved');
  } catch (error) {
    console.error('Failed to save configuration:', error);
  } finally {
    isSaving.value = false;
  }
};

// Load README when modal opens or addon changes
watch(() => props.addon, async (newAddon) => {
  if (!newAddon || !props.open) return;

  isLoadingReadme.value = true;
  try {
    const branch = newAddon.definition.branch || 'main';
    readme.value = await TauriAPI.fetchGithubReadme(
      newAddon.definition.github_owner,
      newAddon.definition.github_repo,
      branch
    );
  } catch (error) {
    console.error('Failed to load README:', error);
    readme.value = '';
  } finally {
    isLoadingReadme.value = false;
  }
}, { immediate: true });
</script>

<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent v-if="addon" class="w-screen h-screen !max-w-screen rounded-none p-0 flex flex-col gap-0">
      <DialogHeader class="shrink-0 px-6 py-4 border-b border-border">
        <DialogTitle class="flex items-center gap-2">
          {{ addon.definition.nice_name }}
          <Badge :class="statusBadgeClass" class="text-xs">
            {{ statusText }}
          </Badge>
        </DialogTitle>
        <DialogDescription>
          {{ addon.definition.description }}
        </DialogDescription>
      </DialogHeader>

      <div class="flex-1 overflow-y-auto px-6 py-4 space-y-4">
        <!-- GitHub Info -->
        <div class="space-y-2">
          <h4 class="text-sm font-semibold text-foreground">Repository</h4>
          <Button
            variant="outline"
            class="w-full justify-start"
            @click="openGithubRepo"
          >
            <ExternalLink :size="16" class="mr-2" />
            {{ addon.definition.github_owner }}/{{ addon.definition.github_repo }}
          </Button>
        </div>

        <Separator />

        <!-- Version Info -->
        <div class="space-y-2">
          <h4 class="text-sm font-semibold text-foreground">Versions</h4>
          <div class="grid grid-cols-2 gap-3">
            <div class="p-3 bg-muted rounded-lg">
              <p class="text-xs text-muted-foreground mb-1">Local</p>
              <p class="text-sm font-medium">
                {{ addon.local_info?.version || 'Not installed' }}
              </p>
            </div>
            <div class="p-3 bg-muted rounded-lg">
              <p class="text-xs text-muted-foreground mb-1">Remote</p>
              <p class="text-sm font-medium">
                {{ addon.remote_version || 'Unknown' }}
              </p>
            </div>
          </div>
        </div>

        <Separator />

        <!-- Update Configuration -->
        <div class="space-y-3">
          <h4 class="text-sm font-semibold text-foreground">Update Configuration</h4>

          <!-- Update Mode Select -->
          <div class="space-y-2">
            <Label class="text-sm text-muted-foreground">Update Mode</Label>
            <Select v-model="updateMode">
              <SelectTrigger>
                <SelectValue placeholder="Select update mode" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="release">Release</SelectItem>
                <SelectItem value="branch">Branch</SelectItem>
              </SelectContent>
            </Select>
            <p class="text-xs text-muted-foreground">
              {{ updateMode === 'release' ? 'Update from latest GitHub release' : 'Update from a specific branch' }}
            </p>
          </div>

          <!-- Branch Select (only if branch mode) -->
          <div v-if="updateMode === 'branch'" class="space-y-2">
            <Label class="text-sm text-muted-foreground">Branch</Label>
            <Select v-model="selectedBranch" :disabled="isLoadingBranches">
              <SelectTrigger>
                <SelectValue :placeholder="isLoadingBranches ? 'Loading branches...' : 'Select branch'" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem v-for="branch in availableBranches" :key="branch" :value="branch">
                  {{ branch }}
                </SelectItem>
              </SelectContent>
            </Select>
            <p class="text-xs text-muted-foreground">
              Current: {{ selectedBranch }}
            </p>
          </div>
        </div>

        <Separator />

        <!-- README -->
        <div class="space-y-2">
          <h4 class="text-sm font-semibold text-foreground">Documentation</h4>
          <div v-if="isLoadingReadme" class="p-4 bg-muted rounded-lg">
            <p class="text-sm text-muted-foreground">Loading README...</p>
          </div>
          <div
            v-else
            class="prose prose-invert prose-sm max-w-none p-4 bg-muted rounded-lg overflow-auto"
            v-html="readmeHtml"
          />
        </div>
      </div>

      <DialogFooter class="shrink-0 px-6 py-4 border-t border-border">
        <div class="flex gap-2 w-full">
          <Button
            type="button"
            variant="outline"
            @click="emit('update:open', false)"
            class="flex-1"
          >
            Close
          </Button>
          <Button
            type="button"
            @click="saveConfiguration"
            :disabled="!hasUnsavedChanges || isSaving"
            class="flex-1"
          >
            {{ isSaving ? 'Saving...' : 'Save Configuration' }}
          </Button>
        </div>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
