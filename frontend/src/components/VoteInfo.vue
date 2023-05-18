<script setup lang="ts">
import { ElectionType } from '@/backend';
import type { PropType } from 'vue';

const props = defineProps({
    election_type: {
        type: Object as PropType<ElectionType>,
        required: true,
    }
});

const info = [
    {
        message: "Rank the candidates in the order you like them.",
        election_type: ElectionType.PreferentialVoting,
    },
    {
        message: "Rank the candidates in the order you like them.",
        election_type: ElectionType.SingleTransferableVote,
    },
    {
        message: "Rank the candidates in the order you like them.",
        election_type: ElectionType.BordaCount,
    },
    {
        message: "Click on the candidates you like.",
        election_type: ElectionType.Approval,
    },
    {
        message: "Give each candidate a score from 0 to 5, like stars.",
        election_type: ElectionType.Star,
    },
    {
        message: "You have some points to give. Give them to your favorite candidates. You can give more points to your most favorite.",
        election_type: ElectionType.Cumulative,
    },
    {
        message: "Pick the candidate you don't like and mark them.",
        election_type: ElectionType.AntiPlurality,
    },
    {
        message: "There's only one candidate. Choose to approve or disapprove them.",
        election_type: ElectionType.SingleParty,
    },
    {
        message: "Rate each candidate as 'Good,' 'OK,' or 'Bad.'",
        election_type: ElectionType.ThreeTwoOne,
    },
    {
        message: "Rank the candidates in the order you like them.",
        election_type: ElectionType.CondorcetMethod,
    },
    {
        message: "Give each candidate a rating.",
        election_type: ElectionType.MajorityJudgment,
    },
    {
        message: "Give each candidate a score higher the better.",
        election_type: ElectionType.Score,
    },
    {
        message: "Give each candidate a rating.",
        election_type: ElectionType.UsualJudgment,
    },
    {
        message: "Select a single candidate to vote for.",
        election_type: ElectionType.SingleNonTransferable,
    }
];

function get_message(): string {
    const result = info.find((element) => {
        return element.election_type === props.election_type;
    });

    if (result) return result.message;

    return 'unknown voting type';
}

</script>

<template>
    <div>
        <h2>⚠️How to Vote in {{ election_type }}⚠️</h2>
        <p>{{ get_message() }}</p>
    </div>
</template>

<style scoped>
p {
    font-weight: 600;
}
</style>
