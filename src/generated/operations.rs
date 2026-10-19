// Generated from the pinned Arbol release by Cloudflare Forge. Do not edit.
use crate::{Effect, Operation, QueryKind};

/// The platform release these operations come from.
pub const RELEASE: &str = "1.20261019.4056";

/// `account.apiKeys.create`: Create an API key
pub const ACCOUNT_API_KEYS_CREATE: Operation = Operation {
    name: "account.apiKeys.create",
    method: "POST",
    path: "/v1/account/api-keys",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `account.apiKeys.delete`: Delete an API key
pub const ACCOUNT_API_KEYS_DELETE: Operation = Operation {
    name: "account.apiKeys.delete",
    method: "DELETE",
    path: "/v1/account/api-keys/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `account.apiKeys.list`: List the API keys
pub const ACCOUNT_API_KEYS_LIST: Operation = Operation {
    name: "account.apiKeys.list",
    method: "GET",
    path: "/v1/account/api-keys",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.apiKeys.rotate`: Rotate an API key
pub const ACCOUNT_API_KEYS_ROTATE: Operation = Operation {
    name: "account.apiKeys.rotate",
    method: "POST",
    path: "/v1/account/api-keys/{id}/rotate",
    path_field: Some("id"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `account.billing.overview`: Get the billing overview
pub const ACCOUNT_BILLING_OVERVIEW: Operation = Operation {
    name: "account.billing.overview",
    method: "GET",
    path: "/v1/account/billing/overview",
    path_field: None,
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.billingProfiles.get`: Get the billing profile
pub const ACCOUNT_BILLING_PROFILES_GET: Operation = Operation {
    name: "account.billingProfiles.get",
    method: "GET",
    path: "/v1/account/billing-profiles/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.billingProfiles.update`: Update the billing profile
pub const ACCOUNT_BILLING_PROFILES_UPDATE: Operation = Operation {
    name: "account.billingProfiles.update",
    method: "PATCH",
    path: "/v1/account/billing-profiles/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `account.connections.create`: Add a connection
pub const ACCOUNT_CONNECTIONS_CREATE: Operation = Operation {
    name: "account.connections.create",
    method: "POST",
    path: "/v1/account/connections",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `account.connections.delete`: Remove a connection
pub const ACCOUNT_CONNECTIONS_DELETE: Operation = Operation {
    name: "account.connections.delete",
    method: "DELETE",
    path: "/v1/account/connections/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `account.connections.list`: List the connections
pub const ACCOUNT_CONNECTIONS_LIST: Operation = Operation {
    name: "account.connections.list",
    method: "GET",
    path: "/v1/account/connections",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 4194304,
};

/// `account.connections.update`: Change a connection
pub const ACCOUNT_CONNECTIONS_UPDATE: Operation = Operation {
    name: "account.connections.update",
    method: "PATCH",
    path: "/v1/account/connections/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `account.invitations.create`: Invite someone to a product
pub const ACCOUNT_INVITATIONS_CREATE: Operation = Operation {
    name: "account.invitations.create",
    method: "POST",
    path: "/v1/account/invitations",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `account.invitations.delete`: Revoke an invitation
pub const ACCOUNT_INVITATIONS_DELETE: Operation = Operation {
    name: "account.invitations.delete",
    method: "DELETE",
    path: "/v1/account/invitations/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `account.invitations.list`: List the invitations
pub const ACCOUNT_INVITATIONS_LIST: Operation = Operation {
    name: "account.invitations.list",
    method: "GET",
    path: "/v1/account/invitations",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.invoices.document`: Download an invoice's PDF
pub const ACCOUNT_INVOICES_DOCUMENT: Operation = Operation {
    name: "account.invoices.document",
    method: "GET",
    path: "/v1/account/invoices/{id}/document",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 16777216,
};

/// `account.invoices.get`: Get an invoice
pub const ACCOUNT_INVOICES_GET: Operation = Operation {
    name: "account.invoices.get",
    method: "GET",
    path: "/v1/account/invoices/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.invoices.list`: List invoices
pub const ACCOUNT_INVOICES_LIST: Operation = Operation {
    name: "account.invoices.list",
    method: "GET",
    path: "/v1/account/invoices",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.members.create`: Give a member a role in a product
pub const ACCOUNT_MEMBERS_CREATE: Operation = Operation {
    name: "account.members.create",
    method: "POST",
    path: "/v1/account/members",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `account.members.delete`: Remove someone's access
pub const ACCOUNT_MEMBERS_DELETE: Operation = Operation {
    name: "account.members.delete",
    method: "DELETE",
    path: "/v1/account/members/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `account.members.list`: List the members
pub const ACCOUNT_MEMBERS_LIST: Operation = Operation {
    name: "account.members.list",
    method: "GET",
    path: "/v1/account/members",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.members.update`: Change someone's role in a product
pub const ACCOUNT_MEMBERS_UPDATE: Operation = Operation {
    name: "account.members.update",
    method: "PATCH",
    path: "/v1/account/members/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `account.organizations.configure`: Configure the organization
pub const ACCOUNT_ORGANIZATIONS_CONFIGURE: Operation = Operation {
    name: "account.organizations.configure",
    method: "POST",
    path: "/v1/account/organizations/configure",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `account.organizations.current`: Get the current organization
pub const ACCOUNT_ORGANIZATIONS_CURRENT: Operation = Operation {
    name: "account.organizations.current",
    method: "GET",
    path: "/v1/account/organizations/current",
    path_field: None,
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.organizations.list`: List my organizations
pub const ACCOUNT_ORGANIZATIONS_LIST: Operation = Operation {
    name: "account.organizations.list",
    method: "GET",
    path: "/v1/account/organizations",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.payments.list`: List payments
pub const ACCOUNT_PAYMENTS_LIST: Operation = Operation {
    name: "account.payments.list",
    method: "GET",
    path: "/v1/account/payments",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.permissions.list`: List the platform permissions
pub const ACCOUNT_PERMISSIONS_LIST: Operation = Operation {
    name: "account.permissions.list",
    method: "GET",
    path: "/v1/account/permissions",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.roles.list`: List the roles of each product
pub const ACCOUNT_ROLES_LIST: Operation = Operation {
    name: "account.roles.list",
    method: "GET",
    path: "/v1/account/roles",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.usage.breakdown`: Break usage down
pub const ACCOUNT_USAGE_BREAKDOWN: Operation = Operation {
    name: "account.usage.breakdown",
    method: "GET",
    path: "/v1/account/usage/breakdown",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("groupBy", QueryKind::String), ("from", QueryKind::String), ("to", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.usage.summary`: Get the usage of a month
pub const ACCOUNT_USAGE_SUMMARY: Operation = Operation {
    name: "account.usage.summary",
    method: "GET",
    path: "/v1/account/usage/summary",
    path_field: None,
    query: Some(&[("period", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `account.walletTransactions.list`: List balance movements
pub const ACCOUNT_WALLET_TRANSACTIONS_LIST: Operation = Operation {
    name: "account.walletTransactions.list",
    method: "GET",
    path: "/v1/account/wallet-transactions",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `agenda.appointments.create`: Create Appointment
pub const AGENDA_APPOINTMENTS_CREATE: Operation = Operation {
    name: "agenda.appointments.create",
    method: "POST",
    path: "/v1/agenda/appointments",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 8388608,
};

/// `agenda.appointments.get`: Get Appointment
pub const AGENDA_APPOINTMENTS_GET: Operation = Operation {
    name: "agenda.appointments.get",
    method: "GET",
    path: "/v1/agenda/appointments/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.appointments.list`: List Appointments
pub const AGENDA_APPOINTMENTS_LIST: Operation = Operation {
    name: "agenda.appointments.list",
    method: "POST",
    path: "/v1/agenda/appointments/search",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.appointments.reschedule`: Reschedule Appointment
pub const AGENDA_APPOINTMENTS_RESCHEDULE: Operation = Operation {
    name: "agenda.appointments.reschedule",
    method: "POST",
    path: "/v1/agenda/appointments/{id}/reschedule",
    path_field: Some("id"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 8388608,
};

/// `agenda.appointments.setStatus`: Update Appointment Status
pub const AGENDA_APPOINTMENTS_SET_STATUS: Operation = Operation {
    name: "agenda.appointments.setStatus",
    method: "PATCH",
    path: "/v1/agenda/appointments/{id}/status",
    path_field: Some("id"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 8388608,
};

/// `agenda.availability.first`: Get First Available
pub const AGENDA_AVAILABILITY_FIRST: Operation = Operation {
    name: "agenda.availability.first",
    method: "GET",
    path: "/v1/agenda/services/{serviceId}/availability/first",
    path_field: Some("serviceId"),
    query: Some(&[("days", QueryKind::String), ("durationMinutes", QueryKind::String), ("from", QueryKind::String), ("occupationMinutes", QueryKind::String), ("group", QueryKind::String), ("limit", QueryKind::Integer), ("locationId", QueryKind::String), ("resourceId", QueryKind::String), ("timezone", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.availability.list`: Get Availability
pub const AGENDA_AVAILABILITY_LIST: Operation = Operation {
    name: "agenda.availability.list",
    method: "POST",
    path: "/v1/agenda/offerings/{offeringId}/availability",
    path_field: Some("offeringId"),
    query: None,
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.cancel`: Cancel an appointment
pub const AGENDA_BOOKINGS_CANCEL: Operation = Operation {
    name: "agenda.bookings.cancel",
    method: "POST",
    path: "/v1/agenda/bookings/{id}/cancel",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.create`: Create an appointment
pub const AGENDA_BOOKINGS_CREATE: Operation = Operation {
    name: "agenda.bookings.create",
    method: "POST",
    path: "/v1/agenda/bookings",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.findNearestSlots`: Find the first available times for a service
pub const AGENDA_BOOKINGS_FIND_NEAREST_SLOTS: Operation = Operation {
    name: "agenda.bookings.findNearestSlots",
    method: "POST",
    path: "/v1/agenda/bookings/find-nearest-slots",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.findSlots`: Find scheduling availability
pub const AGENDA_BOOKINGS_FIND_SLOTS: Operation = Operation {
    name: "agenda.bookings.findSlots",
    method: "POST",
    path: "/v1/agenda/bookings/find-slots",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.get`: Get an appointment
pub const AGENDA_BOOKINGS_GET: Operation = Operation {
    name: "agenda.bookings.get",
    method: "GET",
    path: "/v1/agenda/bookings/{id}",
    path_field: Some("id"),
    query: Some(&[("channel", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.getRequirements`: Get booking intake settings
pub const AGENDA_BOOKINGS_GET_REQUIREMENTS: Operation = Operation {
    name: "agenda.bookings.getRequirements",
    method: "GET",
    path: "/v1/agenda/bookings/get-requirements",
    path_field: None,
    query: Some(&[("payerQuery", QueryKind::String), ("channel", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.list`: List a patient's appointments
pub const AGENDA_BOOKINGS_LIST: Operation = Operation {
    name: "agenda.bookings.list",
    method: "GET",
    path: "/v1/agenda/bookings",
    path_field: None,
    query: Some(&[("document", QueryKind::String), ("documentType", QueryKind::String), ("channel", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.listLocations`: List sites
pub const AGENDA_BOOKINGS_LIST_LOCATIONS: Operation = Operation {
    name: "agenda.bookings.listLocations",
    method: "GET",
    path: "/v1/agenda/bookings/list-locations",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("channel", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.reschedule`: Reschedule an appointment
pub const AGENDA_BOOKINGS_RESCHEDULE: Operation = Operation {
    name: "agenda.bookings.reschedule",
    method: "POST",
    path: "/v1/agenda/bookings/{id}/reschedule",
    path_field: Some("id"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 8388608,
};

/// `agenda.bookings.searchOfferings`: Search bookable offerings
pub const AGENDA_BOOKINGS_SEARCH_OFFERINGS: Operation = Operation {
    name: "agenda.bookings.searchOfferings",
    method: "POST",
    path: "/v1/agenda/bookings/search-offerings",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.locations.list`: List Locations
pub const AGENDA_LOCATIONS_LIST: Operation = Operation {
    name: "agenda.locations.list",
    method: "GET",
    path: "/v1/agenda/locations",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.offerings.list`: List Offerings
pub const AGENDA_OFFERINGS_LIST: Operation = Operation {
    name: "agenda.offerings.list",
    method: "GET",
    path: "/v1/agenda/offerings",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("serviceId", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.resources.list`: List Resources
pub const AGENDA_RESOURCES_LIST: Operation = Operation {
    name: "agenda.resources.list",
    method: "GET",
    path: "/v1/agenda/resources",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("kind", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.services.list`: List Services
pub const AGENDA_SERVICES_LIST: Operation = Operation {
    name: "agenda.services.list",
    method: "GET",
    path: "/v1/agenda/services",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 8388608,
};

/// `agenda.waitlist.join`: Add a patient to the waiting list
pub const AGENDA_WAITLIST_JOIN: Operation = Operation {
    name: "agenda.waitlist.join",
    method: "POST",
    path: "/v1/agenda/waitlist/join",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 8388608,
};

/// `axxa.authorizations.lookup`: Look up medical authorizations
pub const AXXA_AUTHORIZATIONS_LOOKUP: Operation = Operation {
    name: "axxa.authorizations.lookup",
    method: "POST",
    path: "/v1/axxa/authorizations/lookup",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `axxa.connections.delete`: Remove a provider connection
pub const AXXA_CONNECTIONS_DELETE: Operation = Operation {
    name: "axxa.connections.delete",
    method: "DELETE",
    path: "/v1/axxa/connections/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `axxa.connections.get`: Get a provider connection
pub const AXXA_CONNECTIONS_GET: Operation = Operation {
    name: "axxa.connections.get",
    method: "GET",
    path: "/v1/axxa/connections/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `axxa.connections.list`: List provider connections
pub const AXXA_CONNECTIONS_LIST: Operation = Operation {
    name: "axxa.connections.list",
    method: "GET",
    path: "/v1/axxa/connections",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `axxa.connections.verify`: Verify a provider connection
pub const AXXA_CONNECTIONS_VERIFY: Operation = Operation {
    name: "axxa.connections.verify",
    method: "POST",
    path: "/v1/axxa/connections/{connectionId}/verify",
    path_field: Some("connectionId"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `axxa.eligibility.lookup`: Look up beneficiary eligibility
pub const AXXA_ELIGIBILITY_LOOKUP: Operation = Operation {
    name: "axxa.eligibility.lookup",
    method: "POST",
    path: "/v1/axxa/eligibility/lookup",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `axxa.provider.get`: Get provider profile
pub const AXXA_PROVIDER_GET: Operation = Operation {
    name: "axxa.provider.get",
    method: "GET",
    path: "/v1/axxa/provider",
    path_field: None,
    query: Some(&[("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.agendaServices.list`: List agenda services
pub const GMEDIC_AGENDA_SERVICES_LIST: Operation = Operation {
    name: "gmedic.agendaServices.list",
    method: "GET",
    path: "/v1/gmedic/agenda-services",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("id", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.agendas.list`: List agendas
pub const GMEDIC_AGENDAS_LIST: Operation = Operation {
    name: "gmedic.agendas.list",
    method: "GET",
    path: "/v1/gmedic/agendas",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("branchOfficeId", QueryKind::String), ("id", QueryKind::String), ("serviceId", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.appointments.create`: Create appointment
pub const GMEDIC_APPOINTMENTS_CREATE: Operation = Operation {
    name: "gmedic.appointments.create",
    method: "POST",
    path: "/v1/gmedic/appointments",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.appointments.list`: List appointments
pub const GMEDIC_APPOINTMENTS_LIST: Operation = Operation {
    name: "gmedic.appointments.list",
    method: "GET",
    path: "/v1/gmedic/appointments",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agendaId", QueryKind::String), ("date", QueryKind::String), ("id", QueryKind::String), ("patientId", QueryKind::String), ("pending", QueryKind::Boolean), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.appointments.update`: Update appointment
pub const GMEDIC_APPOINTMENTS_UPDATE: Operation = Operation {
    name: "gmedic.appointments.update",
    method: "PATCH",
    path: "/v1/gmedic/appointments",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.availability.list`: List availability
pub const GMEDIC_AVAILABILITY_LIST: Operation = Operation {
    name: "gmedic.availability.list",
    method: "GET",
    path: "/v1/gmedic/availability",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agendaId", QueryKind::String), ("endDate", QueryKind::String), ("startDate", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.availableDays.list`: List available days
pub const GMEDIC_AVAILABLE_DAYS_LIST: Operation = Operation {
    name: "gmedic.availableDays.list",
    method: "GET",
    path: "/v1/gmedic/available-days",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agendaId", QueryKind::String), ("month", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.branchOffices.list`: List branch offices
pub const GMEDIC_BRANCH_OFFICES_LIST: Operation = Operation {
    name: "gmedic.branchOffices.list",
    method: "GET",
    path: "/v1/gmedic/branch-offices",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("id", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.connections.delete`: Remove a provider connection
pub const GMEDIC_CONNECTIONS_DELETE: Operation = Operation {
    name: "gmedic.connections.delete",
    method: "DELETE",
    path: "/v1/gmedic/connections/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `gmedic.connections.get`: Get a provider connection
pub const GMEDIC_CONNECTIONS_GET: Operation = Operation {
    name: "gmedic.connections.get",
    method: "GET",
    path: "/v1/gmedic/connections/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.connections.list`: List provider connections
pub const GMEDIC_CONNECTIONS_LIST: Operation = Operation {
    name: "gmedic.connections.list",
    method: "GET",
    path: "/v1/gmedic/connections",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.connections.verify`: Verify a provider connection
pub const GMEDIC_CONNECTIONS_VERIFY: Operation = Operation {
    name: "gmedic.connections.verify",
    method: "POST",
    path: "/v1/gmedic/connections/{connectionId}/verify",
    path_field: Some("connectionId"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.dictionaryEntries.list`: List dictionary entrys
pub const GMEDIC_DICTIONARY_ENTRIES_LIST: Operation = Operation {
    name: "gmedic.dictionaryEntries.list",
    method: "GET",
    path: "/v1/gmedic/dictionary-entries",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("class", QueryKind::String), ("id", QueryKind::String), ("name", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.healthPartners.list`: List health partners
pub const GMEDIC_HEALTH_PARTNERS_LIST: Operation = Operation {
    name: "gmedic.healthPartners.list",
    method: "GET",
    path: "/v1/gmedic/health-partners",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("code", QueryKind::String), ("id", QueryKind::String), ("name", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.healthPromotionEntities.list`: List health promotion entitys
pub const GMEDIC_HEALTH_PROMOTION_ENTITIES_LIST: Operation = Operation {
    name: "gmedic.healthPromotionEntities.list",
    method: "GET",
    path: "/v1/gmedic/health-promotion-entities",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("code", QueryKind::String), ("id", QueryKind::String), ("name", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.laboralRiskManagers.list`: List laboral risk managers
pub const GMEDIC_LABORAL_RISK_MANAGERS_LIST: Operation = Operation {
    name: "gmedic.laboralRiskManagers.list",
    method: "GET",
    path: "/v1/gmedic/laboral-risk-managers",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("code", QueryKind::String), ("id", QueryKind::String), ("name", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.occupations.list`: List occupations
pub const GMEDIC_OCCUPATIONS_LIST: Operation = Operation {
    name: "gmedic.occupations.list",
    method: "GET",
    path: "/v1/gmedic/occupations",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("code", QueryKind::String), ("id", QueryKind::String), ("name", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.patientNotes.create`: Create patient note
pub const GMEDIC_PATIENT_NOTES_CREATE: Operation = Operation {
    name: "gmedic.patientNotes.create",
    method: "POST",
    path: "/v1/gmedic/patient-notes",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.patientNotes.list`: List patient notes
pub const GMEDIC_PATIENT_NOTES_LIST: Operation = Operation {
    name: "gmedic.patientNotes.list",
    method: "GET",
    path: "/v1/gmedic/patient-notes",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("id", QueryKind::String), ("patientId", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.patientNotes.update`: Update patient note
pub const GMEDIC_PATIENT_NOTES_UPDATE: Operation = Operation {
    name: "gmedic.patientNotes.update",
    method: "PATCH",
    path: "/v1/gmedic/patient-notes",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.patients.create`: Create patient
pub const GMEDIC_PATIENTS_CREATE: Operation = Operation {
    name: "gmedic.patients.create",
    method: "POST",
    path: "/v1/gmedic/patients",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.patients.list`: List patients
pub const GMEDIC_PATIENTS_LIST: Operation = Operation {
    name: "gmedic.patients.list",
    method: "GET",
    path: "/v1/gmedic/patients",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("id", QueryKind::String), ("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.patients.update`: Update patient
pub const GMEDIC_PATIENTS_UPDATE: Operation = Operation {
    name: "gmedic.patients.update",
    method: "PATCH",
    path: "/v1/gmedic/patients",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.services.list`: List services
pub const GMEDIC_SERVICES_LIST: Operation = Operation {
    name: "gmedic.services.list",
    method: "GET",
    path: "/v1/gmedic/services",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("id", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `gmedic.sessionPackages.create`: Create session package
pub const GMEDIC_SESSION_PACKAGES_CREATE: Operation = Operation {
    name: "gmedic.sessionPackages.create",
    method: "POST",
    path: "/v1/gmedic/sessions",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `gmedic.sessionPackages.list`: List session packages
pub const GMEDIC_SESSION_PACKAGES_LIST: Operation = Operation {
    name: "gmedic.sessionPackages.list",
    method: "GET",
    path: "/v1/gmedic/sessions",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("authorization", QueryKind::String), ("id", QueryKind::String), ("patientId", QueryKind::String), ("connectionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `nueva-eps.affiliations.lookup`: Look up member affiliation
pub const NUEVA_EPS_AFFILIATIONS_LOOKUP: Operation = Operation {
    name: "nueva-eps.affiliations.lookup",
    method: "POST",
    path: "/v1/nueva-eps/affiliations/lookup",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `nueva-eps.connections.delete`: Remove a provider connection
pub const NUEVA_EPS_CONNECTIONS_DELETE: Operation = Operation {
    name: "nueva-eps.connections.delete",
    method: "DELETE",
    path: "/v1/nueva-eps/connections/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `nueva-eps.connections.get`: Get a provider connection
pub const NUEVA_EPS_CONNECTIONS_GET: Operation = Operation {
    name: "nueva-eps.connections.get",
    method: "GET",
    path: "/v1/nueva-eps/connections/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `nueva-eps.connections.list`: List provider connections
pub const NUEVA_EPS_CONNECTIONS_LIST: Operation = Operation {
    name: "nueva-eps.connections.list",
    method: "GET",
    path: "/v1/nueva-eps/connections",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `nueva-eps.connections.verify`: Verify a provider connection
pub const NUEVA_EPS_CONNECTIONS_VERIFY: Operation = Operation {
    name: "nueva-eps.connections.verify",
    method: "POST",
    path: "/v1/nueva-eps/connections/{connectionId}/verify",
    path_field: Some("connectionId"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `raiz.agents.create`: Create an Agent
pub const RAIZ_AGENTS_CREATE: Operation = Operation {
    name: "raiz.agents.create",
    method: "POST",
    path: "/v1/raiz/agents/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.agents.get`: Get an Agent
pub const RAIZ_AGENTS_GET: Operation = Operation {
    name: "raiz.agents.get",
    method: "GET",
    path: "/v1/raiz/agents/get",
    path_field: None,
    query: Some(&[("agentId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.agents.list`: List Agents
pub const RAIZ_AGENTS_LIST: Operation = Operation {
    name: "raiz.agents.list",
    method: "GET",
    path: "/v1/raiz/agents",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("modality", QueryKind::String), ("q", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.agents.update`: Update an Agent
pub const RAIZ_AGENTS_UPDATE: Operation = Operation {
    name: "raiz.agents.update",
    method: "PATCH",
    path: "/v1/raiz/agents/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.campaigns.create`: Create a call campaign
pub const RAIZ_CAMPAIGNS_CREATE: Operation = Operation {
    name: "raiz.campaigns.create",
    method: "POST",
    path: "/v1/raiz/campaigns/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.campaigns.get`: Get a campaign
pub const RAIZ_CAMPAIGNS_GET: Operation = Operation {
    name: "raiz.campaigns.get",
    method: "GET",
    path: "/v1/raiz/campaigns/get",
    path_field: None,
    query: Some(&[("campaignId", QueryKind::String), ("expand", QueryKind::String), ("recipientsCursor", QueryKind::String), ("recipientsDirection", QueryKind::String), ("recipientsLimit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.campaigns.list`: List campaigns
pub const RAIZ_CAMPAIGNS_LIST: Operation = Operation {
    name: "raiz.campaigns.list",
    method: "GET",
    path: "/v1/raiz/campaigns",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agentId", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.campaigns.update`: Update a campaign
pub const RAIZ_CAMPAIGNS_UPDATE: Operation = Operation {
    name: "raiz.campaigns.update",
    method: "PATCH",
    path: "/v1/raiz/campaigns/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.changes.list`: List changes (audit log)
pub const RAIZ_CHANGES_LIST: Operation = Operation {
    name: "raiz.changes.list",
    method: "GET",
    path: "/v1/raiz/changes",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("authorId", QueryKind::String), ("contactId", QueryKind::String), ("createdAfter", QueryKind::String), ("createdBefore", QueryKind::String), ("resourceId", QueryKind::String), ("resourceType", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.channels.get`: Get a channel
pub const RAIZ_CHANNELS_GET: Operation = Operation {
    name: "raiz.channels.get",
    method: "GET",
    path: "/v1/raiz/channels/get",
    path_field: None,
    query: Some(&[("channelId", QueryKind::String), ("expand", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.channels.list`: List channels
pub const RAIZ_CHANNELS_LIST: Operation = Operation {
    name: "raiz.channels.list",
    method: "GET",
    path: "/v1/raiz/channels",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agentId", QueryKind::String), ("channelType", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.channels.update`: Update a channel
pub const RAIZ_CHANNELS_UPDATE: Operation = Operation {
    name: "raiz.channels.update",
    method: "PATCH",
    path: "/v1/raiz/channels/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.channels.verify`: Verify a channel
pub const RAIZ_CHANNELS_VERIFY: Operation = Operation {
    name: "raiz.channels.verify",
    method: "POST",
    path: "/v1/raiz/channels/verify",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.contacts.count`: Count contacts
pub const RAIZ_CONTACTS_COUNT: Operation = Operation {
    name: "raiz.contacts.count",
    method: "GET",
    path: "/v1/raiz/contacts/count",
    path_field: None,
    query: Some(&[("agentId", QueryKind::String), ("channelType", QueryKind::String), ("handling", QueryKind::String), ("lastContactedAfter", QueryKind::String), ("q", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.contacts.create`: Create a contact
pub const RAIZ_CONTACTS_CREATE: Operation = Operation {
    name: "raiz.contacts.create",
    method: "POST",
    path: "/v1/raiz/contacts/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.contacts.get`: Get a contact
pub const RAIZ_CONTACTS_GET: Operation = Operation {
    name: "raiz.contacts.get",
    method: "GET",
    path: "/v1/raiz/contacts/get",
    path_field: None,
    query: Some(&[("contactId", QueryKind::String), ("expand", QueryKind::String), ("timelineCursor", QueryKind::String), ("timelineDirection", QueryKind::String), ("timelineLimit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.contacts.import`: Import contacts from a CSV
pub const RAIZ_CONTACTS_IMPORT: Operation = Operation {
    name: "raiz.contacts.import",
    method: "POST",
    path: "/v1/raiz/contacts/import",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.contacts.list`: List contacts
pub const RAIZ_CONTACTS_LIST: Operation = Operation {
    name: "raiz.contacts.list",
    method: "GET",
    path: "/v1/raiz/contacts",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agentId", QueryKind::String), ("channelType", QueryKind::String), ("handling", QueryKind::String), ("lastContactedAfter", QueryKind::String), ("q", QueryKind::String), ("sort", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.contacts.update`: Update a contact
pub const RAIZ_CONTACTS_UPDATE: Operation = Operation {
    name: "raiz.contacts.update",
    method: "PATCH",
    path: "/v1/raiz/contacts/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.conversations.continue`: Give a conversation back to the AI to continue
pub const RAIZ_CONVERSATIONS_CONTINUE: Operation = Operation {
    name: "raiz.conversations.continue",
    method: "POST",
    path: "/v1/raiz/conversations/continue",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.conversations.count`: Count conversations
pub const RAIZ_CONVERSATIONS_COUNT: Operation = Operation {
    name: "raiz.conversations.count",
    method: "GET",
    path: "/v1/raiz/conversations/count",
    path_field: None,
    query: Some(&[("agentId", QueryKind::String), ("campaignId", QueryKind::String), ("channelId", QueryKind::String), ("channelType", QueryKind::String), ("contactId", QueryKind::String), ("conversationDirection", QueryKind::String), ("handling", QueryKind::String), ("worker", QueryKind::String), ("origin", QueryKind::String), ("outcome", QueryKind::String), ("metadataReference", QueryKind::String), ("profileId", QueryKind::String), ("q", QueryKind::String), ("requestedBy", QueryKind::String), ("startedAfter", QueryKind::String), ("startedBefore", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.conversations.get`: Get a conversation
pub const RAIZ_CONVERSATIONS_GET: Operation = Operation {
    name: "raiz.conversations.get",
    method: "GET",
    path: "/v1/raiz/conversations/get",
    path_field: None,
    query: Some(&[("conversationId", QueryKind::String), ("expand", QueryKind::String), ("messagesCursor", QueryKind::String), ("messagesDirection", QueryKind::String), ("messagesLimit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.conversations.list`: List conversations
pub const RAIZ_CONVERSATIONS_LIST: Operation = Operation {
    name: "raiz.conversations.list",
    method: "GET",
    path: "/v1/raiz/conversations",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agentId", QueryKind::String), ("campaignId", QueryKind::String), ("channelId", QueryKind::String), ("channelType", QueryKind::String), ("contactId", QueryKind::String), ("conversationDirection", QueryKind::String), ("handling", QueryKind::String), ("worker", QueryKind::String), ("origin", QueryKind::String), ("outcome", QueryKind::String), ("metadataReference", QueryKind::String), ("profileId", QueryKind::String), ("q", QueryKind::String), ("requestedBy", QueryKind::String), ("startedAfter", QueryKind::String), ("startedBefore", QueryKind::String), ("status", QueryKind::String), ("sort", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.conversations.update`: Update a conversation
pub const RAIZ_CONVERSATIONS_UPDATE: Operation = Operation {
    name: "raiz.conversations.update",
    method: "PATCH",
    path: "/v1/raiz/conversations/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.definitions.create`: Create a data definition
pub const RAIZ_DEFINITIONS_CREATE: Operation = Operation {
    name: "raiz.definitions.create",
    method: "POST",
    path: "/v1/raiz/definitions/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.definitions.get`: Get a data definition
pub const RAIZ_DEFINITIONS_GET: Operation = Operation {
    name: "raiz.definitions.get",
    method: "GET",
    path: "/v1/raiz/definitions/get",
    path_field: None,
    query: Some(&[("definitionId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.definitions.list`: List data definitions
pub const RAIZ_DEFINITIONS_LIST: Operation = Operation {
    name: "raiz.definitions.list",
    method: "GET",
    path: "/v1/raiz/definitions",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("kind", QueryKind::String), ("status", QueryKind::String), ("subject", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.definitions.update`: Update a data definition
pub const RAIZ_DEFINITIONS_UPDATE: Operation = Operation {
    name: "raiz.definitions.update",
    method: "PATCH",
    path: "/v1/raiz/definitions/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.evalCases.list`: List evaluation cases
pub const RAIZ_EVAL_CASES_LIST: Operation = Operation {
    name: "raiz.evalCases.list",
    method: "GET",
    path: "/v1/raiz/eval-cases",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agentId", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.evalCases.update`: Review an evaluation case
pub const RAIZ_EVAL_CASES_UPDATE: Operation = Operation {
    name: "raiz.evalCases.update",
    method: "PATCH",
    path: "/v1/raiz/eval-cases/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.evalRuns.create`: Replay evaluation cases on a candidate
pub const RAIZ_EVAL_RUNS_CREATE: Operation = Operation {
    name: "raiz.evalRuns.create",
    method: "POST",
    path: "/v1/raiz/eval-runs/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.evalRuns.get`: Get an evaluation run
pub const RAIZ_EVAL_RUNS_GET: Operation = Operation {
    name: "raiz.evalRuns.get",
    method: "GET",
    path: "/v1/raiz/eval-runs/get",
    path_field: None,
    query: Some(&[("evalRunId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.facts.create`: Record a fact
pub const RAIZ_FACTS_CREATE: Operation = Operation {
    name: "raiz.facts.create",
    method: "POST",
    path: "/v1/raiz/facts/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.facts.update`: Review a fact
pub const RAIZ_FACTS_UPDATE: Operation = Operation {
    name: "raiz.facts.update",
    method: "PATCH",
    path: "/v1/raiz/facts/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.knowledge.create`: Add a Knowledge document
pub const RAIZ_KNOWLEDGE_CREATE: Operation = Operation {
    name: "raiz.knowledge.create",
    method: "POST",
    path: "/v1/raiz/knowledge/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.knowledge.get`: Get a Knowledge document
pub const RAIZ_KNOWLEDGE_GET: Operation = Operation {
    name: "raiz.knowledge.get",
    method: "GET",
    path: "/v1/raiz/knowledge/get",
    path_field: None,
    query: Some(&[("expand", QueryKind::String), ("knowledgeDocumentId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.knowledge.list`: List Knowledge documents
pub const RAIZ_KNOWLEDGE_LIST: Operation = Operation {
    name: "raiz.knowledge.list",
    method: "GET",
    path: "/v1/raiz/knowledge",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("indexStatus", QueryKind::String), ("kind", QueryKind::String), ("q", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.knowledge.search`: Search Knowledge
pub const RAIZ_KNOWLEDGE_SEARCH: Operation = Operation {
    name: "raiz.knowledge.search",
    method: "POST",
    path: "/v1/raiz/knowledge/search",
    path_field: None,
    query: None,
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.knowledge.update`: Update a Knowledge document
pub const RAIZ_KNOWLEDGE_UPDATE: Operation = Operation {
    name: "raiz.knowledge.update",
    method: "PATCH",
    path: "/v1/raiz/knowledge/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.members.create`: Invite a team member
pub const RAIZ_MEMBERS_CREATE: Operation = Operation {
    name: "raiz.members.create",
    method: "POST",
    path: "/v1/raiz/members/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.members.list`: List team members
pub const RAIZ_MEMBERS_LIST: Operation = Operation {
    name: "raiz.members.list",
    method: "GET",
    path: "/v1/raiz/members",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.members.update`: Change a member's roles
pub const RAIZ_MEMBERS_UPDATE: Operation = Operation {
    name: "raiz.members.update",
    method: "PATCH",
    path: "/v1/raiz/members/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.organization.configure`: Update the organization
pub const RAIZ_ORGANIZATION_CONFIGURE: Operation = Operation {
    name: "raiz.organization.configure",
    method: "PATCH",
    path: "/v1/raiz/organization/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.organization.current`: Get the organization
pub const RAIZ_ORGANIZATION_CURRENT: Operation = Operation {
    name: "raiz.organization.current",
    method: "GET",
    path: "/v1/raiz/organization",
    path_field: None,
    query: Some(&[("expand", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.organization.sendDailyReport`: Send the daily report now
pub const RAIZ_ORGANIZATION_SEND_DAILY_REPORT: Operation = Operation {
    name: "raiz.organization.sendDailyReport",
    method: "POST",
    path: "/v1/raiz/organization/send-daily-report",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.profiles.create`: Create a contact profile
pub const RAIZ_PROFILES_CREATE: Operation = Operation {
    name: "raiz.profiles.create",
    method: "POST",
    path: "/v1/raiz/profiles/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.profiles.get`: Get a contact profile
pub const RAIZ_PROFILES_GET: Operation = Operation {
    name: "raiz.profiles.get",
    method: "GET",
    path: "/v1/raiz/profiles/get",
    path_field: None,
    query: Some(&[("profileId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.profiles.list`: List contact profiles
pub const RAIZ_PROFILES_LIST: Operation = Operation {
    name: "raiz.profiles.list",
    method: "GET",
    path: "/v1/raiz/profiles",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("agentId", QueryKind::String), ("channelType", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.profiles.update`: Update a contact profile
pub const RAIZ_PROFILES_UPDATE: Operation = Operation {
    name: "raiz.profiles.update",
    method: "PATCH",
    path: "/v1/raiz/profiles/update",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.reports.current`: Get a report
pub const RAIZ_REPORTS_CURRENT: Operation = Operation {
    name: "raiz.reports.current",
    method: "GET",
    path: "/v1/raiz/reports",
    path_field: None,
    query: Some(&[("agentId", QueryKind::String), ("campaignId", QueryKind::String), ("channelType", QueryKind::String), ("definitionId", QueryKind::String), ("from", QueryKind::String), ("name", QueryKind::String), ("to", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.roles.list`: List roles
pub const RAIZ_ROLES_LIST: Operation = Operation {
    name: "raiz.roles.list",
    method: "GET",
    path: "/v1/raiz/roles",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.suppressions.create`: Block contact
pub const RAIZ_SUPPRESSIONS_CREATE: Operation = Operation {
    name: "raiz.suppressions.create",
    method: "POST",
    path: "/v1/raiz/suppressions/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.suppressions.list`: List blocked contacts
pub const RAIZ_SUPPRESSIONS_LIST: Operation = Operation {
    name: "raiz.suppressions.list",
    method: "GET",
    path: "/v1/raiz/suppressions",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("kind", QueryKind::String), ("phone", QueryKind::String), ("q", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.tasks.cancel`: Cancel a task
pub const RAIZ_TASKS_CANCEL: Operation = Operation {
    name: "raiz.tasks.cancel",
    method: "POST",
    path: "/v1/raiz/tasks/cancel",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.tasks.get`: Get a task
pub const RAIZ_TASKS_GET: Operation = Operation {
    name: "raiz.tasks.get",
    method: "GET",
    path: "/v1/raiz/tasks/get",
    path_field: None,
    query: Some(&[("taskId", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.tasks.list`: List background tasks
pub const RAIZ_TASKS_LIST: Operation = Operation {
    name: "raiz.tasks.list",
    method: "GET",
    path: "/v1/raiz/tasks",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("kind", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.tasks.retry`: Retry a task
pub const RAIZ_TASKS_RETRY: Operation = Operation {
    name: "raiz.tasks.retry",
    method: "POST",
    path: "/v1/raiz/tasks/retry",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.templates.create`: Create a WhatsApp template
pub const RAIZ_TEMPLATES_CREATE: Operation = Operation {
    name: "raiz.templates.create",
    method: "POST",
    path: "/v1/raiz/templates/create",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `raiz.templates.list`: List WhatsApp templates
pub const RAIZ_TEMPLATES_LIST: Operation = Operation {
    name: "raiz.templates.list",
    method: "GET",
    path: "/v1/raiz/templates",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("channelId", QueryKind::String), ("status", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `raiz.voices.list`: List voices
pub const RAIZ_VOICES_LIST: Operation = Operation {
    name: "raiz.voices.list",
    method: "GET",
    path: "/v1/raiz/voices",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("language", QueryKind::String), ("q", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `sura.affiliations.lookup`: Look up eligibility
pub const SURA_AFFILIATIONS_LOOKUP: Operation = Operation {
    name: "sura.affiliations.lookup",
    method: "GET",
    path: "/v1/sura/affiliations/lookup",
    path_field: None,
    query: Some(&[("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.appointments.cancel`: Cancel an appointment
pub const SURA_APPOINTMENTS_CANCEL: Operation = Operation {
    name: "sura.appointments.cancel",
    method: "POST",
    path: "/v1/sura/appointments/cancel",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `sura.appointments.create`: Book an appointment
pub const SURA_APPOINTMENTS_CREATE: Operation = Operation {
    name: "sura.appointments.create",
    method: "POST",
    path: "/v1/sura/appointments",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `sura.appointments.list`: List appointments
pub const SURA_APPOINTMENTS_LIST: Operation = Operation {
    name: "sura.appointments.list",
    method: "GET",
    path: "/v1/sura/appointments",
    path_field: None,
    query: Some(&[("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.appointments.prepare`: Prepare an appointment
pub const SURA_APPOINTMENTS_PREPARE: Operation = Operation {
    name: "sura.appointments.prepare",
    method: "GET",
    path: "/v1/sura/appointments/prepare",
    path_field: None,
    query: Some(&[("areaId", QueryKind::String), ("date", QueryKind::String), ("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("allowOtherProfessionals", QueryKind::String), ("fromDate", QueryKind::String), ("timeFrom", QueryKind::String), ("timeUntil", QueryKind::String), ("limit", QueryKind::String), ("latestDate", QueryKind::String), ("mode", QueryKind::String), ("professional", QueryKind::String), ("serviceId", QueryKind::String), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.authorizations.lookup`: Look up an authorization
pub const SURA_AUTHORIZATIONS_LOOKUP: Operation = Operation {
    name: "sura.authorizations.lookup",
    method: "GET",
    path: "/v1/sura/authorizations/lookup",
    path_field: None,
    query: Some(&[("authorizationNumber", QueryKind::String), ("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("issuerCode", QueryKind::String), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.availability.list`: Find availability
pub const SURA_AVAILABILITY_LIST: Operation = Operation {
    name: "sura.availability.list",
    method: "GET",
    path: "/v1/sura/availability",
    path_field: None,
    query: Some(&[("areaId", QueryKind::String), ("date", QueryKind::String), ("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("professional", QueryKind::String), ("serviceId", QueryKind::String), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.connections.delete`: Remove a SURA connection
pub const SURA_CONNECTIONS_DELETE: Operation = Operation {
    name: "sura.connections.delete",
    method: "DELETE",
    path: "/v1/sura/connections/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 262144,
};

/// `sura.connections.get`: Get a SURA connection
pub const SURA_CONNECTIONS_GET: Operation = Operation {
    name: "sura.connections.get",
    method: "GET",
    path: "/v1/sura/connections/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.connections.list`: List the SURA connections
pub const SURA_CONNECTIONS_LIST: Operation = Operation {
    name: "sura.connections.list",
    method: "GET",
    path: "/v1/sura/connections",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.connections.verify`: Verify a SURA connection
pub const SURA_CONNECTIONS_VERIFY: Operation = Operation {
    name: "sura.connections.verify",
    method: "POST",
    path: "/v1/sura/connections/{connectionId}/verify",
    path_field: Some("connectionId"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 262144,
};

/// `sura.copays.list`: List copay history
pub const SURA_COPAYS_LIST: Operation = Operation {
    name: "sura.copays.list",
    method: "GET",
    path: "/v1/sura/copays",
    path_field: None,
    query: Some(&[("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("year", QueryKind::String), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.operations.get`: Get an operation
pub const SURA_OPERATIONS_GET: Operation = Operation {
    name: "sura.operations.get",
    method: "GET",
    path: "/v1/sura/operations/{operationId}",
    path_field: Some("operationId"),
    query: Some(&[("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.patients.lookup`: Look up a patient
pub const SURA_PATIENTS_LOOKUP: Operation = Operation {
    name: "sura.patients.lookup",
    method: "GET",
    path: "/v1/sura/patients/lookup",
    path_field: None,
    query: Some(&[("documentNumber", QueryKind::String), ("documentType", QueryKind::String), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `sura.services.list`: List services
pub const SURA_SERVICES_LIST: Operation = Operation {
    name: "sura.services.list",
    method: "GET",
    path: "/v1/sura/services",
    path_field: None,
    query: Some(&[("areaId", QueryKind::String), ("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 262144,
};

/// `visoft.agencies.list`: List the clinic's locations
pub const VISOFT_AGENCIES_LIST: Operation = Operation {
    name: "visoft.agencies.list",
    method: "GET",
    path: "/v1/visoft/agencies",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.appointments.cancel`: Cancel an appointment
pub const VISOFT_APPOINTMENTS_CANCEL: Operation = Operation {
    name: "visoft.appointments.cancel",
    method: "POST",
    path: "/v1/visoft/appointments/cancel",
    path_field: None,
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 1048576,
};

/// `visoft.appointments.create`: Book an appointment
pub const VISOFT_APPOINTMENTS_CREATE: Operation = Operation {
    name: "visoft.appointments.create",
    method: "POST",
    path: "/v1/visoft/appointments",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `visoft.availability.list`: Find available appointments
pub const VISOFT_AVAILABILITY_LIST: Operation = Operation {
    name: "visoft.availability.list",
    method: "GET",
    path: "/v1/visoft/availability",
    path_field: None,
    query: Some(&[("documentNumber", QueryKind::String), ("agency", QueryKind::String), ("contract", QueryKind::String), ("service", QueryKind::String), ("date", QueryKind::String), ("medic", QueryKind::String), ("from", QueryKind::String), ("to", QueryKind::String), ("refresh", QueryKind::Boolean), ("specialty", QueryKind::String), ("regimen", QueryKind::String), ("eps", QueryKind::String), ("sala", QueryKind::String), ("modality", QueryKind::String), ("authorizationService", QueryKind::String), ("timeFrom", QueryKind::String), ("timeTo", QueryKind::String), ("intent", QueryKind::String), ("patientAgeMonths", QueryKind::String), ("fonoGroup", QueryKind::String), ("toGroup", QueryKind::String), ("pairWithNext", QueryKind::Boolean), ("professionalChangeConfirmed", QueryKind::Boolean), ("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.clinics.get`: Get a connection's clinic
pub const VISOFT_CLINICS_GET: Operation = Operation {
    name: "visoft.clinics.get",
    method: "GET",
    path: "/v1/visoft/clinics/{connectionId}",
    path_field: Some("connectionId"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.clinics.list`: List the Visoft clinics
pub const VISOFT_CLINICS_LIST: Operation = Operation {
    name: "visoft.clinics.list",
    method: "GET",
    path: "/v1/visoft/clinics",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.connections.delete`: Remove a Visoft connection
pub const VISOFT_CONNECTIONS_DELETE: Operation = Operation {
    name: "visoft.connections.delete",
    method: "DELETE",
    path: "/v1/visoft/connections/{id}",
    path_field: Some("id"),
    query: None,
    effect: Effect::Delete,
    max_output_bytes: 1048576,
};

/// `visoft.connections.get`: Get a Visoft connection
pub const VISOFT_CONNECTIONS_GET: Operation = Operation {
    name: "visoft.connections.get",
    method: "GET",
    path: "/v1/visoft/connections/{id}",
    path_field: Some("id"),
    query: Some(&[]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.connections.list`: List the Visoft connections
pub const VISOFT_CONNECTIONS_LIST: Operation = Operation {
    name: "visoft.connections.list",
    method: "GET",
    path: "/v1/visoft/connections",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.connections.verify`: Verify a Visoft connection
pub const VISOFT_CONNECTIONS_VERIFY: Operation = Operation {
    name: "visoft.connections.verify",
    method: "POST",
    path: "/v1/visoft/connections/{connectionId}/verify",
    path_field: Some("connectionId"),
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `visoft.contracts.list`: List the clinic's contracts
pub const VISOFT_CONTRACTS_LIST: Operation = Operation {
    name: "visoft.contracts.list",
    method: "GET",
    path: "/v1/visoft/contracts",
    path_field: None,
    query: Some(&[("agency", QueryKind::String), ("search", QueryKind::String), ("refresh", QueryKind::Boolean), ("eps", QueryKind::String), ("regimen", QueryKind::String), ("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.groups.list`: List therapy groups
pub const VISOFT_GROUPS_LIST: Operation = Operation {
    name: "visoft.groups.list",
    method: "GET",
    path: "/v1/visoft/groups",
    path_field: None,
    query: Some(&[("specialty", QueryKind::String), ("name", QueryKind::String), ("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.patients.correct`: Correct a patient's details
pub const VISOFT_PATIENTS_CORRECT: Operation = Operation {
    name: "visoft.patients.correct",
    method: "POST",
    path: "/v1/visoft/patients/correct",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `visoft.patients.create`: Register a patient
pub const VISOFT_PATIENTS_CREATE: Operation = Operation {
    name: "visoft.patients.create",
    method: "POST",
    path: "/v1/visoft/patients",
    path_field: None,
    query: None,
    effect: Effect::Write,
    max_output_bytes: 1048576,
};

/// `visoft.patients.lookup`: Look up a patient
pub const VISOFT_PATIENTS_LOOKUP: Operation = Operation {
    name: "visoft.patients.lookup",
    method: "GET",
    path: "/v1/visoft/patients/lookup",
    path_field: None,
    query: Some(&[("documentNumber", QueryKind::String), ("agency", QueryKind::String), ("includePast", QueryKind::Boolean), ("pastDays", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.services.list`: List a contract's services
pub const VISOFT_SERVICES_LIST: Operation = Operation {
    name: "visoft.services.list",
    method: "GET",
    path: "/v1/visoft/services",
    path_field: None,
    query: Some(&[("code", QueryKind::String), ("agency", QueryKind::String), ("search", QueryKind::String), ("refresh", QueryKind::Boolean), ("specialty", QueryKind::String), ("intent", QueryKind::String), ("modality", QueryKind::String), ("authorizationService", QueryKind::String), ("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.specialtyRules.get`: Get a specialty's rules
pub const VISOFT_SPECIALTY_RULES_GET: Operation = Operation {
    name: "visoft.specialtyRules.get",
    method: "GET",
    path: "/v1/visoft/specialty-rules/{name}",
    path_field: Some("name"),
    query: Some(&[("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};

/// `visoft.tools.list`: List the clinic's tools
pub const VISOFT_TOOLS_LIST: Operation = Operation {
    name: "visoft.tools.list",
    method: "GET",
    path: "/v1/visoft/tools",
    path_field: None,
    query: Some(&[("cursor", QueryKind::String), ("direction", QueryKind::String), ("limit", QueryKind::Integer), ("connectionId", QueryKind::String), ("channel", QueryKind::String)]),
    effect: Effect::Read,
    max_output_bytes: 1048576,
};
