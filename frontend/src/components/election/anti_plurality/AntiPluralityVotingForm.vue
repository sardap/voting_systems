<script setup lang="ts">
import { ref, type PropType } from 'vue';
import { type GenericElection, submit_anti_plurality_vote } from '@/backend';
import type { VoteOption } from '@/utils';
import CheckboxChoice from '@/components/voting/CheckboxChoice.vue';

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true,
  },
  options: {
    type: Array as PropType<VoteOption[]>,
    required: true,
  },
  vote_token: {
    type: String,
    required: false,
  }
});

const emits = defineEmits({
  complete: () => true,
  error: (error: string) => true,
});

const loading = ref<boolean>(false);
const options = ref(props.options.map((option) => {
  return {
    ...option,
    approve: false,
  };
}));

async function submit() {
  const selected_voters = options.value.filter(option => option.approve);

  if (selected_voters.length == 0) {
    emits(`error`, `You must select one option`);
    return;
  }

  if (selected_voters.length > 1) {
    emits(`error`, `You can only select a single option`);
    return;
  }

  loading.value = true;
  const response = await submit_anti_plurality_vote(props.election.id, props.vote_token, selected_voters[0].index);
  const data = await response.text();
  loading.value = false;
  if (!response.ok) {
    emits(`error`, data);
    return;
  }

  emits(`complete`);
}
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <CheckboxChoice v-model="options" :single_option="true" />
    <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
  </div>
</template>

<style scoped></style>
