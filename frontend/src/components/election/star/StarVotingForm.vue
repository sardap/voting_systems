<script setup lang="ts">
import { ref, type PropType } from 'vue'
import { ElectionType, submit_generic_vote, type GenericElection } from '@/backend'
import type { VoteOption } from '@/utils'

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true
  },
  options: {
    type: Array as PropType<VoteOption[]>,
    required: true
  },
  vote_token: {
    type: String,
    required: false
  }
})

const emits = defineEmits({
  complete: () => true,
  error: (error: string) => true
})
interface StarVoteOption extends VoteOption {
  points: number
}

const loading = ref<boolean>(false)
const options = ref<StarVoteOption[]>(
  props.options.map((option) => {
    return {
      ...option,
      points: 0
    }
  })
)

async function submit() {
  const candidates = props.election.options

  const vote: number[] = []
  for (let option_index = 0; option_index < candidates.length; option_index++) {
    const approve = options.value.find((option) => option.index == option_index)?.points as number
    vote.push(approve)
  }

  loading.value = true

  const response = await submit_generic_vote(
    ElectionType.Star,
    props.election.id,
    props.vote_token,
    vote
  )
  const data = await response.text()
  loading.value = false
  if (!response.ok) {
    emits(`error`, data)
    console.log(`ERROR: ${data}`)
    return
  }

  emits(`complete`)
}
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <div>
      <div ref="el" class="table">
        <div class="row header">
          <div class="col">
            <p>Name</p>
          </div>
          <div class="col">
            <p>🌟Stars🌟</p>
          </div>
        </div>
        <div v-for="option in options" :key="option.index" class="row">
          <div class="col">
            <p>{{ option.name }}</p>
          </div>
          <div class="col">
            <button
              v-for="i in 6"
              @click="option.points = i - 1"
              :class="option.points == i - 1 ? `star-btn selected` : `star-btn`"
              :key="i"
            >
              <span>{{ i - 1 }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>
    <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
  </div>
</template>

<style scoped>
.star-btn {
  display: inline-block;
  background-color: white;
  padding: 10px;
  cursor: pointer;
  font-size: 18px;
  width: 60px;
  height: 60px;
  line-height: 60px;
  text-align: center;
  -webkit-clip-path: polygon(
    50% 0%,
    61% 35%,
    98% 35%,
    68% 57%,
    79% 91%,
    50% 70%,
    21% 91%,
    32% 57%,
    2% 35%,
    39% 35%
  );
  clip-path: polygon(
    50% 0%,
    61% 35%,
    98% 35%,
    68% 57%,
    79% 91%,
    50% 70%,
    21% 91%,
    32% 57%,
    2% 35%,
    39% 35%
  );
  overflow: visible;
  position: relative;
}

.star-btn span {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1;
}

.selected {
  background-color: gold;
}

div.row:hover {
  background-color: #e8ab8c;
}

div.row:nth-child(2n):hover {
  background-color: #e8ab8c;
}

div.row:nth-child(2n) {
  background-color: #fad8c0;
}

div.row {
  background-color: #fde5d0;
}

div.row.header {
  background-color: #a2f4b9;
}

div.table {
  display: table;
  max-width: 800px;
  width: 70%;
  border-collapse: separate;
  border-spacing: 0;
}

div.row {
  display: table-row;
}

div.col {
  display: table-cell;
  border: 1px solid black;
  padding: 8px;
  text-align: left;
  vertical-align: top;
}

div.col.rank {
  width: 10px;
  text-align: center;
}

.submit {
  margin-top: 20px;
  max-width: 800px;
  width: 70%;
  height: 80px;
}
</style>
