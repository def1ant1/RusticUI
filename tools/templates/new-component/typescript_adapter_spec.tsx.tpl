/**
 * TypeScript smoke test scaffold for the `{{component_pascal}}` Storybook adapter.
 *
 * Shipping a `.spec.tsx` file with the generator guarantees new components join
 * the existing mocha type-check pipeline automatically. Replace the placeholder
 * expectations once the real adapter lands, but keep the automation-root
 * assertion so the docs stub, Storybook story, and Rust modules keep a shared
 * contract.
 */
import { expectType } from '@mui/types';
import { create{{component_pascal}}AdapterTelemetry } from './RusticAdapter';

const telemetry = create{{component_pascal}}AdapterTelemetry();

// Type-level guardrail: the automation root must remain a string literal so the
// docs frontmatter and Rust constants stay synchronised.
expectType<'{{automation_id}}', typeof telemetry.automationRoot>(telemetry.automationRoot);

// Runtime placeholder to remind contributors to replace this file with real
// assertions. Keeping an exported constant avoids unused-variable warnings while
// ensuring CI still loads the module.
export const TODO_REPLACE_WITH_REAL_TESTS = telemetry.notes;
