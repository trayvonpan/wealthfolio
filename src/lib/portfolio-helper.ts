import { AccountSummary, Goal, GoalAllocation, GoalProgress, Holding } from './types';

export function calculateGoalProgress(
  accounts: AccountSummary[],
  goals: Goal[],
  allocations: GoalAllocation[],
): GoalProgress[] {
  // Extract base currency from the first account's performance, or default to 'USD'
  const baseCurrency = accounts[0]?.performance?.baseCurrency || 'USD';

  // Create a map of accountId to marketValue for quick lookup
  const accountValueMap = new Map<string, number>();
  accounts.forEach((account) => {
    accountValueMap.set(
      account.account.id,
      account?.performance?.totalValue * (account?.performance?.exchangeRate || 1),
    );
  });

  // Sort goals by targetValue
  goals.sort((a, b) => a.targetAmount - b.targetAmount);

  return goals.map((goal) => {
    const goalAllocations = allocations.filter((allocation) => allocation.goalId === goal.id) || [];
    const totalAllocatedValue = goalAllocations.reduce((total, allocation) => {
      const accountValue = accountValueMap.get(allocation.accountId) || 0;
      const allocatedValue = (accountValue * allocation.percentAllocation) / 100;
      return total + allocatedValue;
    }, 0);

    // Calculate progress
    const progress = goal.targetAmount > 0 ? (totalAllocatedValue / goal.targetAmount) * 100 : 0;

    return {
      name: goal.title,
      targetValue: goal.targetAmount,
      currentValue: totalAllocatedValue,
      progress: progress,
      currency: baseCurrency,
    };
  });
}
