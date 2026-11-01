// Generated from the pinned Arbol release by Cloudflare Forge. Do not edit.
#![allow(clippy::all, clippy::pedantic)]

use crate::generated::{operations, types};
use crate::{Client, Error};

impl Client {
    pub fn account(&self) -> AccountApi<'_> {
        AccountApi(self)
    }

    pub fn agenda(&self) -> AgendaApi<'_> {
        AgendaApi(self)
    }

    pub fn axxa(&self) -> AxxaApi<'_> {
        AxxaApi(self)
    }

    pub fn gmedic(&self) -> GmedicApi<'_> {
        GmedicApi(self)
    }

    pub fn nueva_eps(&self) -> NuevaEpsApi<'_> {
        NuevaEpsApi(self)
    }

    pub fn orsovision(&self) -> OrsovisionApi<'_> {
        OrsovisionApi(self)
    }

    pub fn raiz(&self) -> RaizApi<'_> {
        RaizApi(self)
    }

    pub fn salud_total(&self) -> SaludTotalApi<'_> {
        SaludTotalApi(self)
    }

    pub fn somos(&self) -> SomosApi<'_> {
        SomosApi(self)
    }

    pub fn sura(&self) -> SuraApi<'_> {
        SuraApi(self)
    }

    pub fn visoft(&self) -> VisoftApi<'_> {
        VisoftApi(self)
    }
}

/// `account.apiKeys`
#[derive(Debug, Clone, Copy)]
pub struct AccountApiKeysApi<'a>(&'a Client);

impl<'a> AccountApiKeysApi<'a> {
    /// Create an API key
    ///
    /// Creates an API key for this organization with the chosen permissions, which must be ones the caller holds, and optionally restricted per integration to some of its tools and connections. Answers the key once.
    pub async fn create(&self, input: &types::AccountApiKeysCreateInput) -> Result<types::AccountApiKeysCreateOutput, Error> {
        self.0.call(&operations::ACCOUNT_API_KEYS_CREATE, input).await
    }

    /// Delete an API key
    ///
    /// Deletes the API key. Tokens it already obtained are refused within about two minutes, even before they expire.
    pub async fn delete(&self, input: &types::AccountApiKeysDeleteInput) -> Result<types::AccountApiKeysDeleteOutput, Error> {
        self.0.call(&operations::ACCOUNT_API_KEYS_DELETE, input).await
    }

    /// List the API keys
    ///
    /// The organization's API keys with the permissions each carries. The keys themselves are never shown again.
    pub async fn list(&self, input: &types::AccountApiKeysListInput) -> Result<types::AccountApiKeysListOutput, Error> {
        self.0.call(&operations::ACCOUNT_API_KEYS_LIST, input).await
    }

    /// Rotate an API key
    ///
    /// Gives the API key a new secret and answers the new key once. The old key stops working within about two minutes.
    pub async fn rotate(&self, input: &types::AccountApiKeysRotateInput) -> Result<types::AccountApiKeysRotateOutput, Error> {
        self.0.call(&operations::ACCOUNT_API_KEYS_ROTATE, input).await
    }
}

/// `account.billing`
#[derive(Debug, Clone, Copy)]
pub struct AccountBillingApi<'a>(&'a Client);

impl<'a> AccountBillingApi<'a> {
    /// Get the billing overview
    ///
    /// The organization's billing status, balance, active plans, allowances and what it spent this month.
    pub async fn get(&self, input: &types::AccountBillingGetInput) -> Result<types::AccountBillingGetOutput, Error> {
        self.0.call(&operations::ACCOUNT_BILLING_GET, input).await
    }
}

/// `account.billingProfiles`
#[derive(Debug, Clone, Copy)]
pub struct AccountBillingProfilesApi<'a>(&'a Client);

impl<'a> AccountBillingProfilesApi<'a> {
    /// Get the billing profile
    ///
    /// The legal data invoices carry: legal name, tax id, billing email and billing day.
    pub async fn get(&self, input: &types::AccountBillingProfilesGetInput) -> Result<types::AccountBillingProfilesGetOutput, Error> {
        self.0.call(&operations::ACCOUNT_BILLING_PROFILES_GET, input).await
    }

    /// Update the billing profile
    ///
    /// Changes the legal data invoices carry. A field left out keeps its value.
    pub async fn update(&self, input: &types::AccountBillingProfilesUpdateInput) -> Result<types::AccountBillingProfilesUpdateOutput, Error> {
        self.0.call(&operations::ACCOUNT_BILLING_PROFILES_UPDATE, input).await
    }
}

/// `account.colombianRecords`
#[derive(Debug, Clone, Copy)]
pub struct AccountColombianRecordsApi<'a>(&'a Client);

impl<'a> AccountColombianRecordsApi<'a> {
    /// Consult an included Colombian source
    ///
    /// Run one included consultation in the signed-in organization.
    pub async fn lookup(&self, input: &types::AccountColombianRecordsLookupInput) -> Result<types::AccountColombianRecordsLookupOutput, Error> {
        self.0.call(&operations::ACCOUNT_COLOMBIAN_RECORDS_LOOKUP, input).await
    }
}

/// `account.connections`
#[derive(Debug, Clone, Copy)]
pub struct AccountConnectionsApi<'a>(&'a Client);

impl<'a> AccountConnectionsApi<'a> {
    /// List the connections
    ///
    /// The organization's integration connections (SURA, Visoft, its Agenda, custom servers) with the tools each offers an agent and their version.
    pub async fn list(&self, input: &types::AccountConnectionsListInput) -> Result<types::AccountConnectionsListOutput, Error> {
        self.0.call(&operations::ACCOUNT_CONNECTIONS_LIST, input).await
    }
}

/// `account.invitations`
#[derive(Debug, Clone, Copy)]
pub struct AccountInvitationsApi<'a>(&'a Client);

impl<'a> AccountInvitationsApi<'a> {
    /// Invite someone to a product
    ///
    /// Invites someone without an Arbol account by email with a role in Raíz, Agenda, Ámbar or Account. The invitation email comes from the product's own application. Existing accounts receive access with `members.create` and sign in with their current password.
    pub async fn create(&self, input: &types::AccountInvitationsCreateInput) -> Result<types::AccountInvitationsCreateOutput, Error> {
        self.0.call(&operations::ACCOUNT_INVITATIONS_CREATE, input).await
    }

    /// Revoke an invitation
    ///
    /// Revokes a pending invitation whose roles all belong to products the caller administers.
    pub async fn delete(&self, input: &types::AccountInvitationsDeleteInput) -> Result<types::AccountInvitationsDeleteOutput, Error> {
        self.0.call(&operations::ACCOUNT_INVITATIONS_DELETE, input).await
    }

    /// List the invitations
    ///
    /// The organization's pending invitations, newest first, and whether the caller may revoke each.
    pub async fn list(&self, input: &types::AccountInvitationsListInput) -> Result<types::AccountInvitationsListOutput, Error> {
        self.0.call(&operations::ACCOUNT_INVITATIONS_LIST, input).await
    }
}

/// `account.invoiceDocuments`
#[derive(Debug, Clone, Copy)]
pub struct AccountInvoiceDocumentsApi<'a>(&'a Client);

impl<'a> AccountInvoiceDocumentsApi<'a> {
    /// Download an invoice's PDF
    ///
    /// The legal invoice document, as base64.
    pub async fn get(&self, input: &types::AccountInvoiceDocumentsGetInput) -> Result<types::AccountInvoiceDocumentsGetOutput, Error> {
        self.0.call(&operations::ACCOUNT_INVOICE_DOCUMENTS_GET, input).await
    }
}

/// `account.invoices`
#[derive(Debug, Clone, Copy)]
pub struct AccountInvoicesApi<'a>(&'a Client);

impl<'a> AccountInvoicesApi<'a> {
    /// Get an invoice
    ///
    /// One invoice with its lines and how to pay it.
    pub async fn get(&self, input: &types::AccountInvoicesGetInput) -> Result<types::AccountInvoicesGetOutput, Error> {
        self.0.call(&operations::ACCOUNT_INVOICES_GET, input).await
    }

    /// List invoices
    ///
    /// The organization's latest 100 invoices, newest first.
    pub async fn list(&self, input: &types::AccountInvoicesListInput) -> Result<types::AccountInvoicesListOutput, Error> {
        self.0.call(&operations::ACCOUNT_INVOICES_LIST, input).await
    }
}

/// `account.members`
#[derive(Debug, Clone, Copy)]
pub struct AccountMembersApi<'a>(&'a Client);

impl<'a> AccountMembersApi<'a> {
    /// Give a member a role in a product
    ///
    /// Gives someone with an existing Arbol account a role in Raíz, Agenda, Ámbar or Account, joining them to this organization when needed. They sign in with their current password. Someone without an account is invited with `invitations.create`.
    pub async fn create(&self, input: &types::AccountMembersCreateInput) -> Result<types::AccountMembersCreateOutput, Error> {
        self.0.call(&operations::ACCOUNT_MEMBERS_CREATE, input).await
    }

    /// Remove someone's access
    ///
    /// Revokes the member's roles in every product the caller administers. They leave the organization only when no other role is left.
    pub async fn delete(&self, input: &types::AccountMembersDeleteInput) -> Result<types::AccountMembersDeleteOutput, Error> {
        self.0.call(&operations::ACCOUNT_MEMBERS_DELETE, input).await
    }

    /// List the members
    ///
    /// The organization's members with their roles in Raíz, Agenda, Ámbar and Account, by name.
    pub async fn list(&self, input: &types::AccountMembersListInput) -> Result<types::AccountMembersListOutput, Error> {
        self.0.call(&operations::ACCOUNT_MEMBERS_LIST, input).await
    }

    /// Change someone's role in a product
    ///
    /// Sets a member's role in Raíz, Agenda, Ámbar or Account, or removes their access to that product with null. Their other roles stay.
    pub async fn update(&self, input: &types::AccountMembersUpdateInput) -> Result<types::AccountMembersUpdateOutput, Error> {
        self.0.call(&operations::ACCOUNT_MEMBERS_UPDATE, input).await
    }
}

/// `account.organization`
#[derive(Debug, Clone, Copy)]
pub struct AccountOrganizationApi<'a>(&'a Client);

impl<'a> AccountOrganizationApi<'a> {
    /// Get the current organization
    ///
    /// The organization the caller's token is for, the caller's roles in each of its products and their permissions in Account.
    pub async fn get(&self, input: &types::AccountOrganizationGetInput) -> Result<types::AccountOrganizationGetOutput, Error> {
        self.0.call(&operations::ACCOUNT_ORGANIZATION_GET, input).await
    }

    /// Update the organization
    ///
    /// Changes the current organization's name, logo and sign-in colors, as people see them in every Arbol application and on its sign-in page.
    pub async fn update(&self, input: &types::AccountOrganizationUpdateInput) -> Result<types::AccountOrganizationUpdateOutput, Error> {
        self.0.call(&operations::ACCOUNT_ORGANIZATION_UPDATE, input).await
    }
}

/// `account.organizations`
#[derive(Debug, Clone, Copy)]
pub struct AccountOrganizationsApi<'a>(&'a Client);

impl<'a> AccountOrganizationsApi<'a> {
    /// List my organizations
    ///
    /// The organizations the signed-in person belongs to, to switch between them.
    pub async fn list(&self, input: &types::AccountOrganizationsListInput) -> Result<types::AccountOrganizationsListOutput, Error> {
        self.0.call(&operations::ACCOUNT_ORGANIZATIONS_LIST, input).await
    }
}

/// `account.payments`
#[derive(Debug, Clone, Copy)]
pub struct AccountPaymentsApi<'a>(&'a Client);

impl<'a> AccountPaymentsApi<'a> {
    /// List payments
    ///
    /// The organization's latest 100 payments.
    pub async fn list(&self, input: &types::AccountPaymentsListInput) -> Result<types::AccountPaymentsListOutput, Error> {
        self.0.call(&operations::ACCOUNT_PAYMENTS_LIST, input).await
    }
}

/// `account.permissions`
#[derive(Debug, Clone, Copy)]
pub struct AccountPermissionsApi<'a>(&'a Client);

impl<'a> AccountPermissionsApi<'a> {
    /// List the platform permissions
    ///
    /// Every permission an API key may carry, by product, and which of them the caller holds. A key never carries a permission its creator lacks.
    pub async fn list(&self, input: &types::AccountPermissionsListInput) -> Result<types::AccountPermissionsListOutput, Error> {
        self.0.call(&operations::ACCOUNT_PERMISSIONS_LIST, input).await
    }
}

/// `account.personalConsents`
#[derive(Debug, Clone, Copy)]
pub struct AccountPersonalConsentsApi<'a>(&'a Client);

impl<'a> AccountPersonalConsentsApi<'a> {
    /// Prepare a personal Colombian Police declaration
    ///
    /// Send the holder’s explicitly accepted document and receive the matching personal-use declaration.
    pub async fn prepare(&self, input: &types::AccountPersonalConsentsPrepareInput) -> Result<types::AccountPersonalConsentsPrepareOutput, Error> {
        self.0.call(&operations::ACCOUNT_PERSONAL_CONSENTS_PREPARE, input).await
    }
}

/// `account.roles`
#[derive(Debug, Clone, Copy)]
pub struct AccountRolesApi<'a>(&'a Client);

impl<'a> AccountRolesApi<'a> {
    /// List the roles of each product
    ///
    /// Each product's roles with their permissions on its API, and whether the caller may grant each product's roles.
    pub async fn list(&self, input: &types::AccountRolesListInput) -> Result<types::AccountRolesListOutput, Error> {
        self.0.call(&operations::ACCOUNT_ROLES_LIST, input).await
    }
}

/// `account.usage`
#[derive(Debug, Clone, Copy)]
pub struct AccountUsageApi<'a>(&'a Client);

impl<'a> AccountUsageApi<'a> {
    /// Break usage down
    ///
    /// Usage and charges in a window of at most 93 days, by meter, agent, model or channel: one row per meter, agent, model or channel, by key.
    pub async fn list(&self, input: &types::AccountUsageListInput) -> Result<types::AccountUsageListOutput, Error> {
        self.0.call(&operations::ACCOUNT_USAGE_LIST, input).await
    }
}

/// `account.usageSummary`
#[derive(Debug, Clone, Copy)]
pub struct AccountUsageSummaryApi<'a>(&'a Client);

impl<'a> AccountUsageSummaryApi<'a> {
    /// Get the usage of a month
    ///
    /// What the organization used and was charged in one calendar month (UTC), by meter.
    pub async fn get(&self, input: &types::AccountUsageSummaryGetInput) -> Result<types::AccountUsageSummaryGetOutput, Error> {
        self.0.call(&operations::ACCOUNT_USAGE_SUMMARY_GET, input).await
    }
}

/// `account.walletTransactions`
#[derive(Debug, Clone, Copy)]
pub struct AccountWalletTransactionsApi<'a>(&'a Client);

impl<'a> AccountWalletTransactionsApi<'a> {
    /// List balance movements
    ///
    /// The latest 100 movements of the organization's balance, newest first.
    pub async fn list(&self, input: &types::AccountWalletTransactionsListInput) -> Result<types::AccountWalletTransactionsListOutput, Error> {
        self.0.call(&operations::ACCOUNT_WALLET_TRANSACTIONS_LIST, input).await
    }
}

/// `account`
#[derive(Debug, Clone, Copy)]
pub struct AccountApi<'a>(&'a Client);

impl<'a> AccountApi<'a> {
    pub fn api_keys(&self) -> AccountApiKeysApi<'a> {
        AccountApiKeysApi(self.0)
    }

    pub fn billing(&self) -> AccountBillingApi<'a> {
        AccountBillingApi(self.0)
    }

    pub fn billing_profiles(&self) -> AccountBillingProfilesApi<'a> {
        AccountBillingProfilesApi(self.0)
    }

    pub fn colombian_records(&self) -> AccountColombianRecordsApi<'a> {
        AccountColombianRecordsApi(self.0)
    }

    pub fn connections(&self) -> AccountConnectionsApi<'a> {
        AccountConnectionsApi(self.0)
    }

    pub fn invitations(&self) -> AccountInvitationsApi<'a> {
        AccountInvitationsApi(self.0)
    }

    pub fn invoice_documents(&self) -> AccountInvoiceDocumentsApi<'a> {
        AccountInvoiceDocumentsApi(self.0)
    }

    pub fn invoices(&self) -> AccountInvoicesApi<'a> {
        AccountInvoicesApi(self.0)
    }

    pub fn members(&self) -> AccountMembersApi<'a> {
        AccountMembersApi(self.0)
    }

    pub fn organization(&self) -> AccountOrganizationApi<'a> {
        AccountOrganizationApi(self.0)
    }

    pub fn organizations(&self) -> AccountOrganizationsApi<'a> {
        AccountOrganizationsApi(self.0)
    }

    pub fn payments(&self) -> AccountPaymentsApi<'a> {
        AccountPaymentsApi(self.0)
    }

    pub fn permissions(&self) -> AccountPermissionsApi<'a> {
        AccountPermissionsApi(self.0)
    }

    pub fn personal_consents(&self) -> AccountPersonalConsentsApi<'a> {
        AccountPersonalConsentsApi(self.0)
    }

    pub fn roles(&self) -> AccountRolesApi<'a> {
        AccountRolesApi(self.0)
    }

    pub fn usage(&self) -> AccountUsageApi<'a> {
        AccountUsageApi(self.0)
    }

    pub fn usage_summary(&self) -> AccountUsageSummaryApi<'a> {
        AccountUsageSummaryApi(self.0)
    }

    pub fn wallet_transactions(&self) -> AccountWalletTransactionsApi<'a> {
        AccountWalletTransactionsApi(self.0)
    }
}

/// `agenda.appointments`
#[derive(Debug, Clone, Copy)]
pub struct AgendaAppointmentsApi<'a>(&'a Client);

impl<'a> AgendaAppointmentsApi<'a> {
    /// Change Appointment Status
    ///
    /// Apply an allowed appointment status transition using the expected version and record its attributed history.
    pub async fn change_status(&self, input: &types::AgendaAppointmentsChangeStatusInput) -> Result<types::AgendaAppointmentsChangeStatusOutput, Error> {
        self.0.call(&operations::AGENDA_APPOINTMENTS_CHANGE_STATUS, input).await
    }

    /// Create Appointment
    ///
    /// Book an admitted slot or convert an active hold, preserving request-key idempotency and serialized capacity checks.
    pub async fn create(&self, input: &types::AgendaAppointmentsCreateInput) -> Result<types::AgendaAppointmentsCreateOutput, Error> {
        self.0.call(&operations::AGENDA_APPOINTMENTS_CREATE, input).await
    }

    /// Get Appointment
    ///
    /// Read one appointment with its service, assigned agendas and patient details within the authenticated tenant.
    pub async fn get(&self, input: &types::AgendaAppointmentsGetInput) -> Result<types::AgendaAppointmentsGetOutput, Error> {
        self.0.call(&operations::AGENDA_APPOINTMENTS_GET, input).await
    }

    /// List Appointments
    ///
    /// List the appointments in an appointment or creation window, filtered by the fields they match, using stable keyset pagination. To find a patient by name or document, search instead.
    pub async fn list(&self, input: &types::AgendaAppointmentsListInput) -> Result<types::AgendaAppointmentsListOutput, Error> {
        self.0.call(&operations::AGENDA_APPOINTMENTS_LIST, input).await
    }

    /// Reschedule Appointment
    ///
    /// Move an appointment to an admitted slot with version and capacity checks, then reconcile its notifications and provider events.
    pub async fn reschedule(&self, input: &types::AgendaAppointmentsRescheduleInput) -> Result<types::AgendaAppointmentsRescheduleOutput, Error> {
        self.0.call(&operations::AGENDA_APPOINTMENTS_RESCHEDULE, input).await
    }

    /// Search Appointments
    ///
    /// Search appointments by patient name or document (q), or within the requested window and filters, using stable keyset pagination.
    pub async fn search(&self, input: &types::AgendaAppointmentsSearchInput) -> Result<types::AgendaAppointmentsSearchOutput, Error> {
        self.0.call(&operations::AGENDA_APPOINTMENTS_SEARCH, input).await
    }
}

/// `agenda.availability`
#[derive(Debug, Clone, Copy)]
pub struct AgendaAvailabilityApi<'a>(&'a Client);

impl<'a> AgendaAvailabilityApi<'a> {
    /// Get Availability
    ///
    /// Find bookable slots for one offering within the requested window, earliest first, a page at a time, enforcing schedule, capacity and active holds.
    pub async fn list(&self, input: &types::AgendaAvailabilityListInput) -> Result<types::AgendaAvailabilityListOutput, Error> {
        self.0.call(&operations::AGENDA_AVAILABILITY_LIST, input).await
    }
}

/// `agenda.bookingLocations`
#[derive(Debug, Clone, Copy)]
pub struct AgendaBookingLocationsApi<'a>(&'a Client);

impl<'a> AgendaBookingLocationsApi<'a> {
    /// List sites
    ///
    /// List the organization's sites (sedes), a page at a time: follow pageInfo.nextCursor for more. Pass a site's id as locationId to find_first_available to search only its agendas.
    pub async fn list(&self, input: &types::AgendaBookingLocationsListInput) -> Result<types::AgendaBookingLocationsListOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKING_LOCATIONS_LIST, input).await
    }
}

/// `agenda.bookingOfferings`
#[derive(Debug, Clone, Copy)]
pub struct AgendaBookingOfferingsApi<'a>(&'a Client);

impl<'a> AgendaBookingOfferingsApi<'a> {
    /// Search bookable offerings
    ///
    /// Search active bookable offerings by q (such as electroencefalograma or neurología), exact serviceId or locationId. Pass the known service name instead of loading the whole catalog. Returns a compact page with offering/service ids, agendas, sites, current bookingChecklist, staff guidance (description), patientInstructions, eligibility (the ages a service takes, in completed years on the appointment day; null for any age) and occupationMinutes (how long the patient stays when longer than the appointment, such as a 12-hour study). Check these requirements against known patient facts before slots or booking; explain patient instructions, and ask only for missing required facts. When eligibility is set, ask the patient's date of birth before booking and pass it as patientBirthDate. If exactly one location applies, use it automatically without asking the patient to choose a site; provide its address for arrival or booking confirmation. Never infer age from a name or document type. A listed service does not establish payer coverage. Follow pageInfo.nextCursor only if needed.
    pub async fn list(&self, input: &types::AgendaBookingOfferingsListInput) -> Result<types::AgendaBookingOfferingsListOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKING_OFFERINGS_LIST, input).await
    }
}

/// `agenda.bookingRequirements`
#[derive(Debug, Clone, Copy)]
pub struct AgendaBookingRequirementsApi<'a>(&'a Client);

impl<'a> AgendaBookingRequirementsApi<'a> {
    /// Get booking intake settings
    ///
    /// Get required/optional booking fields, document-type codes and active payers. Pass payerQuery when a document already identifies the payer (e.g. the health plan on an insurance card), to omit unrelated payers. Reuse clear patient data from the conversation or documents instead of asking it again; request only missing required or ambiguous values. Each payer includes attended sites and sitesNote: a nonempty sites list restricts which IPS/sede is served; an empty list means all its sites. Each payer also has bookingPolicy: 'staff_only' means only the clinic's team books its patients, so never book them; use add_to_waitlist with reason payer_review instead. An empty payer match does not establish coverage.
    pub async fn get(&self, input: &types::AgendaBookingRequirementsGetInput) -> Result<types::AgendaBookingRequirementsGetOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKING_REQUIREMENTS_GET, input).await
    }
}

/// `agenda.bookingSlots`
#[derive(Debug, Clone, Copy)]
pub struct AgendaBookingSlotsApi<'a>(&'a Client);

impl<'a> AgendaBookingSlotsApi<'a> {
    /// Find scheduling availability
    ///
    /// Find available slots for one offering, earliest first, a page at a time: follow pageInfo.nextCursor for later ones. The returned slotToken is required to create the appointment.
    pub async fn list(&self, input: &types::AgendaBookingSlotsListInput) -> Result<types::AgendaBookingSlotsListOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKING_SLOTS_LIST, input).await
    }
}

/// `agenda.bookings`
#[derive(Debug, Clone, Copy)]
pub struct AgendaBookingsApi<'a>(&'a Client);

impl<'a> AgendaBookingsApi<'a> {
    /// Cancel an appointment
    ///
    /// Cancel one appointment after the person explicitly confirms which one. Frees its slot. Pass the appointment as appointmentId (its id from list_patient_appointments) and the reason the person gave.
    pub async fn cancel(&self, input: &types::AgendaBookingsCancelInput) -> Result<types::AgendaBookingsCancelOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKINGS_CANCEL, input).await
    }

    /// Create an appointment
    ///
    /// Book one slot only after the person explicitly confirms it. Pass the exact slotToken returned by find_availability, the patient's name, documentNumber and documentType (one of the organization's documentTypes codes; the first one when omitted), and the intake the organization requires (call get_intake_settings first). When the service has an age rule (eligibility in list_offerings), ask the patient's date of birth and pass patientBirthDate; never infer age from a name or document type. The booking also creates or updates the patient's record. It answers with guidance to tell the patient and slip: a link, valid for an hour, to the appointment's printable slip PDF (with the service's indications) to send them as a file.
    pub async fn create(&self, input: &types::AgendaBookingsCreateInput) -> Result<types::AgendaBookingsCreateOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKINGS_CREATE, input).await
    }

    /// Get an appointment
    ///
    /// Read one appointment by its id, appointmentId: when it starts and ends, its status, the patient, the agendas (resourceIds), the intake and guidance (the service's current patientInstructions, arrivalOrder, who attends it and the site with its address) to tell the patient, and slip: a link, valid for an hour, to its printable slip PDF to send them as a file.
    pub async fn get(&self, input: &types::AgendaBookingsGetInput) -> Result<types::AgendaBookingsGetOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKINGS_GET, input).await
    }

    /// Look up a patient's appointments
    ///
    /// Find a patient by their exact documentNumber (and documentType, such as CC or TI, when known) and list their appointments, newest first, each with id, start, status and agendas; upcoming ones also carry guidance (patientInstructions, arrivalOrder, who attends and the site's address). Use it before cancelling or rescheduling; found=false means no patient has that document. When several patients share the number under different types, patients lists them and appointments is empty: ask which one and call again with documentType.
    pub async fn lookup(&self, input: &types::AgendaBookingsLookupInput) -> Result<types::AgendaBookingsLookupOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKINGS_LOOKUP, input).await
    }

    /// Reschedule an appointment
    ///
    /// Move one appointment to a new time after the person confirms it. First find the new time with find_availability or find_first_available for the same service (another professional or room of that service is fine), then pass that slotToken and the appointment as appointmentId. A slot of another service is refused: the appointment keeps its service.
    pub async fn reschedule(&self, input: &types::AgendaBookingsRescheduleInput) -> Result<types::AgendaBookingsRescheduleOutput, Error> {
        self.0.call(&operations::AGENDA_BOOKINGS_RESCHEDULE, input).await
    }
}

/// `agenda.firstAvailability`
#[derive(Debug, Clone, Copy)]
pub struct AgendaFirstAvailabilityApi<'a>(&'a Client);

impl<'a> AgendaFirstAvailabilityApi<'a> {
    /// Get First Available
    ///
    /// Find the earliest bookable slots for a service across eligible agendas, with a bounded search horizon and continuation.
    pub async fn get(&self, input: &types::AgendaFirstAvailabilityGetInput) -> Result<types::AgendaFirstAvailabilityGetOutput, Error> {
        self.0.call(&operations::AGENDA_FIRST_AVAILABILITY_GET, input).await
    }
}

/// `agenda.locations`
#[derive(Debug, Clone, Copy)]
pub struct AgendaLocationsApi<'a>(&'a Client);

impl<'a> AgendaLocationsApi<'a> {
    /// List Locations
    ///
    /// List clinic locations and their active state within the authenticated tenant.
    pub async fn list(&self, input: &types::AgendaLocationsListInput) -> Result<types::AgendaLocationsListOutput, Error> {
        self.0.call(&operations::AGENDA_LOCATIONS_LIST, input).await
    }
}

/// `agenda.nearestBookingSlots`
#[derive(Debug, Clone, Copy)]
pub struct AgendaNearestBookingSlotsApi<'a>(&'a Client);

impl<'a> AgendaNearestBookingSlotsApi<'a> {
    /// Find the first available times for a service
    ///
    /// Find the truly nearest bookable times for one service, in ONE call, across every professional, room or equipment that offers it (up to 50 agendas). Leave days out: the search then reaches as far as the service can be booked (its booking horizon, up to a year) and stops at the first `limit` slots, so an opening months away is found at once. Do not page week by week. An empty data list means there is no opening within that reach (searchedUntil says how far it looked); offer the waitlist instead. Pass days only to look at a shorter stretch. group 'time' (default) returns the earliest slots overall, a page at a time: pageInfo.nextCursor reads the slots after these, and pageInfo.previousCursor with direction backward the ones before; group 'offering' returns each agenda's first slots as one page. Use limit 1–3 to quote the nearest time. Each slotToken books with create_appointment.
    pub async fn list(&self, input: &types::AgendaNearestBookingSlotsListInput) -> Result<types::AgendaNearestBookingSlotsListOutput, Error> {
        self.0.call(&operations::AGENDA_NEAREST_BOOKING_SLOTS_LIST, input).await
    }
}

/// `agenda.offerings`
#[derive(Debug, Clone, Copy)]
pub struct AgendaOfferingsApi<'a>(&'a Client);

impl<'a> AgendaOfferingsApi<'a> {
    /// Create Offering
    ///
    /// Attach a service to an agenda with its scheduling requirements, duration, eligibility and intake policy.
    pub async fn create(&self, input: &types::AgendaOfferingsCreateInput) -> Result<types::AgendaOfferingsCreateOutput, Error> {
        self.0.call(&operations::AGENDA_OFFERINGS_CREATE, input).await
    }

    /// List Offerings
    ///
    /// List service offerings using tenant-qualified agenda, service and active-state filters.
    pub async fn list(&self, input: &types::AgendaOfferingsListInput) -> Result<types::AgendaOfferingsListOutput, Error> {
        self.0.call(&operations::AGENDA_OFFERINGS_LIST, input).await
    }

    /// Update Offering
    ///
    /// Update the offering named by offeringId using its expected version.
    pub async fn update(&self, input: &types::AgendaOfferingsUpdateInput) -> Result<types::AgendaOfferingsUpdateOutput, Error> {
        self.0.call(&operations::AGENDA_OFFERINGS_UPDATE, input).await
    }
}

/// `agenda.resourceSchedules`
#[derive(Debug, Clone, Copy)]
pub struct AgendaResourceSchedulesApi<'a>(&'a Client);

impl<'a> AgendaResourceSchedulesApi<'a> {
    /// Update Resource Schedule
    ///
    /// Update an agenda’s hours as one complete set of working rules, replaced atomically, naming the rules last read (expectedRules); a rule left out of the set is removed.
    pub async fn update(&self, input: &types::AgendaResourceSchedulesUpdateInput) -> Result<types::AgendaResourceSchedulesUpdateOutput, Error> {
        self.0.call(&operations::AGENDA_RESOURCE_SCHEDULES_UPDATE, input).await
    }
}

/// `agenda.resources`
#[derive(Debug, Clone, Copy)]
pub struct AgendaResourcesApi<'a>(&'a Client);

impl<'a> AgendaResourcesApi<'a> {
    /// Create Resource
    ///
    /// Create a professional, room or equipment agenda with its capacity and booking policy.
    pub async fn create(&self, input: &types::AgendaResourcesCreateInput) -> Result<types::AgendaResourcesCreateOutput, Error> {
        self.0.call(&operations::AGENDA_RESOURCES_CREATE, input).await
    }

    /// List Resources
    ///
    /// List agendas using kind and active-state filters with stable keyset pagination.
    pub async fn list(&self, input: &types::AgendaResourcesListInput) -> Result<types::AgendaResourcesListOutput, Error> {
        self.0.call(&operations::AGENDA_RESOURCES_LIST, input).await
    }

    /// Update Resource
    ///
    /// Update the agenda named by resourceId using its expected version.
    pub async fn update(&self, input: &types::AgendaResourcesUpdateInput) -> Result<types::AgendaResourcesUpdateOutput, Error> {
        self.0.call(&operations::AGENDA_RESOURCES_UPDATE, input).await
    }
}

/// `agenda.scheduleRules`
#[derive(Debug, Clone, Copy)]
pub struct AgendaScheduleRulesApi<'a>(&'a Client);

impl<'a> AgendaScheduleRulesApi<'a> {
    /// Create Schedule Rule
    ///
    /// Add one working rule to an agenda: a weekly band, a dated override, time off or a block, with its window as from and to in the agenda’s timezone.
    pub async fn create(&self, input: &types::AgendaScheduleRulesCreateInput) -> Result<types::AgendaScheduleRulesCreateOutput, Error> {
        self.0.call(&operations::AGENDA_SCHEDULE_RULES_CREATE, input).await
    }

    /// List Schedule Rules
    ///
    /// Read the agenda’s weekly bands and dated overrides in its local timezone, by agenda then start, a page at a time.
    pub async fn list(&self, input: &types::AgendaScheduleRulesListInput) -> Result<types::AgendaScheduleRulesListOutput, Error> {
        self.0.call(&operations::AGENDA_SCHEDULE_RULES_LIST, input).await
    }

    /// Update Schedule Rule
    ///
    /// Rewrite one of an agenda’s working rules, named by scheduleRuleId, with the version last read (expectedVersion). A rule stays on its agenda.
    pub async fn update(&self, input: &types::AgendaScheduleRulesUpdateInput) -> Result<types::AgendaScheduleRulesUpdateOutput, Error> {
        self.0.call(&operations::AGENDA_SCHEDULE_RULES_UPDATE, input).await
    }
}

/// `agenda.services`
#[derive(Debug, Clone, Copy)]
pub struct AgendaServicesApi<'a>(&'a Client);

impl<'a> AgendaServicesApi<'a> {
    /// Create Service
    ///
    /// Create a bookable service with its intake, eligibility and duration policy.
    pub async fn create(&self, input: &types::AgendaServicesCreateInput) -> Result<types::AgendaServicesCreateOutput, Error> {
        self.0.call(&operations::AGENDA_SERVICES_CREATE, input).await
    }

    /// List Services
    ///
    /// List services using tenant-qualified filters and stable keyset pagination.
    pub async fn list(&self, input: &types::AgendaServicesListInput) -> Result<types::AgendaServicesListOutput, Error> {
        self.0.call(&operations::AGENDA_SERVICES_LIST, input).await
    }

    /// Update Service
    ///
    /// Update the service named by serviceId using its expected version.
    pub async fn update(&self, input: &types::AgendaServicesUpdateInput) -> Result<types::AgendaServicesUpdateOutput, Error> {
        self.0.call(&operations::AGENDA_SERVICES_UPDATE, input).await
    }
}

/// `agenda.waitlist`
#[derive(Debug, Clone, Copy)]
pub struct AgendaWaitlistApi<'a>(&'a Client);

impl<'a> AgendaWaitlistApi<'a> {
    /// Add a patient to the waiting list
    ///
    /// Put a patient on the clinic's waiting list for the team to act on, only with the person's consent. Use reason 'payer_review' instead of booking when the patient's payer (EPS) has bookingPolicy 'staff_only' in get_intake_settings (create_appointment refuses them): the team coordinates with the doctor and books it. Use reason 'priority' right after create_appointment when the patient asked for a priority (urgent) appointment: pass that appointment's id as appointmentId so the team can bring it forward. For agent calls, the platform supplies conversationUrl from the invoking conversation; omit that field. Direct callers may supply the referring conversation link so the team can reopen it. Pass the patient's name, documentNumber and documentType, phone, serviceId and the payer as the patient said it. Use resourceId only for the professional the person requested. Calling again for the same patient, service and reason returns the same open entry and updates supplied consented preferences, note, payer, conversation or appointment association. A supplied confirmed phone replaces the patient contact phone; omitted or null phone preserves it. Omitted fields preserve their values; resourceId:null explicitly accepts any professional. When supplying preferences, send the complete consented date/time window; null fields clear those limits. Identical repeats do not change the version. Staff status, priority and original queue position are preserved. A closed entry is not reopened. This registers a waiting-list request, never an appointment. Then tell the patient the clinic's team will contact them.
    pub async fn join(&self, input: &types::AgendaWaitlistJoinInput) -> Result<types::AgendaWaitlistJoinOutput, Error> {
        self.0.call(&operations::AGENDA_WAITLIST_JOIN, input).await
    }

    /// List Waitlist
    ///
    /// List the organization’s waitlist entries, a page at a time: open ones first (urgent, then the longest waiting), then closed ones, most recently changed first, up to 500 in all. Narrow them by serviceId and resourceId (the agenda an entry asked for); offeringId finds only entries from before services and agendas were chosen separately.
    pub async fn list(&self, input: &types::AgendaWaitlistListInput) -> Result<types::AgendaWaitlistListOutput, Error> {
        self.0.call(&operations::AGENDA_WAITLIST_LIST, input).await
    }
}

/// `agenda`
#[derive(Debug, Clone, Copy)]
pub struct AgendaApi<'a>(&'a Client);

impl<'a> AgendaApi<'a> {
    pub fn appointments(&self) -> AgendaAppointmentsApi<'a> {
        AgendaAppointmentsApi(self.0)
    }

    pub fn availability(&self) -> AgendaAvailabilityApi<'a> {
        AgendaAvailabilityApi(self.0)
    }

    pub fn booking_locations(&self) -> AgendaBookingLocationsApi<'a> {
        AgendaBookingLocationsApi(self.0)
    }

    pub fn booking_offerings(&self) -> AgendaBookingOfferingsApi<'a> {
        AgendaBookingOfferingsApi(self.0)
    }

    pub fn booking_requirements(&self) -> AgendaBookingRequirementsApi<'a> {
        AgendaBookingRequirementsApi(self.0)
    }

    pub fn booking_slots(&self) -> AgendaBookingSlotsApi<'a> {
        AgendaBookingSlotsApi(self.0)
    }

    pub fn bookings(&self) -> AgendaBookingsApi<'a> {
        AgendaBookingsApi(self.0)
    }

    pub fn first_availability(&self) -> AgendaFirstAvailabilityApi<'a> {
        AgendaFirstAvailabilityApi(self.0)
    }

    pub fn locations(&self) -> AgendaLocationsApi<'a> {
        AgendaLocationsApi(self.0)
    }

    pub fn nearest_booking_slots(&self) -> AgendaNearestBookingSlotsApi<'a> {
        AgendaNearestBookingSlotsApi(self.0)
    }

    pub fn offerings(&self) -> AgendaOfferingsApi<'a> {
        AgendaOfferingsApi(self.0)
    }

    pub fn resource_schedules(&self) -> AgendaResourceSchedulesApi<'a> {
        AgendaResourceSchedulesApi(self.0)
    }

    pub fn resources(&self) -> AgendaResourcesApi<'a> {
        AgendaResourcesApi(self.0)
    }

    pub fn schedule_rules(&self) -> AgendaScheduleRulesApi<'a> {
        AgendaScheduleRulesApi(self.0)
    }

    pub fn services(&self) -> AgendaServicesApi<'a> {
        AgendaServicesApi(self.0)
    }

    pub fn waitlist(&self) -> AgendaWaitlistApi<'a> {
        AgendaWaitlistApi(self.0)
    }
}

/// `axxa.authorizations`
#[derive(Debug, Clone, Copy)]
pub struct AxxaAuthorizationsApi<'a>(&'a Client);

impl<'a> AxxaAuthorizationsApi<'a> {
    /// Look up medical authorizations
    ///
    /// Read authorizations for the supplied beneficiary document. Only a successful empty array means no matches; provider errors do not establish whether authorizations exist.
    pub async fn lookup(&self, input: &types::AxxaAuthorizationsLookupInput) -> Result<types::AxxaAuthorizationsLookupOutput, Error> {
        self.0.call(&operations::AXXA_AUTHORIZATIONS_LOOKUP, input).await
    }
}

/// `axxa.connections`
#[derive(Debug, Clone, Copy)]
pub struct AxxaConnectionsApi<'a>(&'a Client);

impl<'a> AxxaConnectionsApi<'a> {
    /// Remove a provider connection
    ///
    /// Remove the provider-owned connection before Account retires its sealed credentials. Login cooldowns remain durable.
    pub async fn delete(&self, input: &types::AxxaConnectionsDeleteInput) -> Result<types::AxxaConnectionsDeleteOutput, Error> {
        self.0.call(&operations::AXXA_CONNECTIONS_DELETE, input).await
    }

    /// Get a provider connection
    ///
    /// One connection owned by the verified organization. Never a credential or a provider request.
    pub async fn get(&self, input: &types::AxxaConnectionsGetInput) -> Result<types::AxxaConnectionsGetOutput, Error> {
        self.0.call(&operations::AXXA_CONNECTIONS_GET, input).await
    }

    /// List provider connections
    ///
    /// The organization’s Account connections, without reading credentials or contacting the provider.
    pub async fn list(&self, input: &types::AxxaConnectionsListInput) -> Result<types::AxxaConnectionsListOutput, Error> {
        self.0.call(&operations::AXXA_CONNECTIONS_LIST, input).await
    }

    /// Verify a provider connection
    ///
    /// Verify these credentials once before Account seals them. A rejected verification changes no active credential. The durable organization-wide budget prevents replacement connections from bypassing login protection.
    pub async fn verify(&self, input: &types::AxxaConnectionsVerifyInput) -> Result<types::AxxaConnectionsVerifyOutput, Error> {
        self.0.call(&operations::AXXA_CONNECTIONS_VERIFY, input).await
    }
}

/// `axxa.eligibility`
#[derive(Debug, Clone, Copy)]
pub struct AxxaEligibilityApi<'a>(&'a Client);

impl<'a> AxxaEligibilityApi<'a> {
    /// Look up beneficiary eligibility
    ///
    /// Read the plans or affiliation validation messages for a document and service category. Plans do not establish coverage for a specific CUPS, authorization, or voucher amount.
    pub async fn lookup(&self, input: &types::AxxaEligibilityLookupInput) -> Result<types::AxxaEligibilityLookupOutput, Error> {
        self.0.call(&operations::AXXA_ELIGIBILITY_LOOKUP, input).await
    }
}

/// `axxa.provider`
#[derive(Debug, Clone, Copy)]
pub struct AxxaProviderApi<'a>(&'a Client);

impl<'a> AxxaProviderApi<'a> {
    /// Get provider profile
    ///
    /// Read the organization's configured AXA provider name, code and branch. Authentication and renewal are internal.
    pub async fn get(&self, input: &types::AxxaProviderGetInput) -> Result<types::AxxaProviderGetOutput, Error> {
        self.0.call(&operations::AXXA_PROVIDER_GET, input).await
    }
}

/// `axxa`
#[derive(Debug, Clone, Copy)]
pub struct AxxaApi<'a>(&'a Client);

impl<'a> AxxaApi<'a> {
    pub fn authorizations(&self) -> AxxaAuthorizationsApi<'a> {
        AxxaAuthorizationsApi(self.0)
    }

    pub fn connections(&self) -> AxxaConnectionsApi<'a> {
        AxxaConnectionsApi(self.0)
    }

    pub fn eligibility(&self) -> AxxaEligibilityApi<'a> {
        AxxaEligibilityApi(self.0)
    }

    pub fn provider(&self) -> AxxaProviderApi<'a> {
        AxxaProviderApi(self.0)
    }
}

/// `gmedic.agendaServices`
#[derive(Debug, Clone, Copy)]
pub struct GmedicAgendaServicesApi<'a>(&'a Client);

impl<'a> GmedicAgendaServicesApi<'a> {
    /// List agenda services
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicAgendaServicesListInput) -> Result<types::GmedicAgendaServicesListOutput, Error> {
        self.0.call(&operations::GMEDIC_AGENDA_SERVICES_LIST, input).await
    }
}

/// `gmedic.agendas`
#[derive(Debug, Clone, Copy)]
pub struct GmedicAgendasApi<'a>(&'a Client);

impl<'a> GmedicAgendasApi<'a> {
    /// List agendas
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicAgendasListInput) -> Result<types::GmedicAgendasListOutput, Error> {
        self.0.call(&operations::GMEDIC_AGENDAS_LIST, input).await
    }
}

/// `gmedic.appointments`
#[derive(Debug, Clone, Copy)]
pub struct GmedicAppointmentsApi<'a>(&'a Client);

impl<'a> GmedicAppointmentsApi<'a> {
    /// Create appointment
    ///
    /// Use the organization gMedic account. No automatic write retries; reconcile an unknown outcome before repeating.
    pub async fn create(&self, input: &types::GmedicAppointmentsCreateInput) -> Result<types::GmedicAppointmentsCreateOutput, Error> {
        self.0.call(&operations::GMEDIC_APPOINTMENTS_CREATE, input).await
    }

    /// List appointments
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicAppointmentsListInput) -> Result<types::GmedicAppointmentsListOutput, Error> {
        self.0.call(&operations::GMEDIC_APPOINTMENTS_LIST, input).await
    }

    /// Update appointment
    ///
    /// Use the organization gMedic account. No automatic write retries; reconcile an unknown outcome before repeating.
    pub async fn update(&self, input: &types::GmedicAppointmentsUpdateInput) -> Result<types::GmedicAppointmentsUpdateOutput, Error> {
        self.0.call(&operations::GMEDIC_APPOINTMENTS_UPDATE, input).await
    }
}

/// `gmedic.availability`
#[derive(Debug, Clone, Copy)]
pub struct GmedicAvailabilityApi<'a>(&'a Client);

impl<'a> GmedicAvailabilityApi<'a> {
    /// List availability
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicAvailabilityListInput) -> Result<types::GmedicAvailabilityListOutput, Error> {
        self.0.call(&operations::GMEDIC_AVAILABILITY_LIST, input).await
    }
}

/// `gmedic.availableDays`
#[derive(Debug, Clone, Copy)]
pub struct GmedicAvailableDaysApi<'a>(&'a Client);

impl<'a> GmedicAvailableDaysApi<'a> {
    /// List available days
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicAvailableDaysListInput) -> Result<types::GmedicAvailableDaysListOutput, Error> {
        self.0.call(&operations::GMEDIC_AVAILABLE_DAYS_LIST, input).await
    }
}

/// `gmedic.branchOffices`
#[derive(Debug, Clone, Copy)]
pub struct GmedicBranchOfficesApi<'a>(&'a Client);

impl<'a> GmedicBranchOfficesApi<'a> {
    /// List branch offices
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicBranchOfficesListInput) -> Result<types::GmedicBranchOfficesListOutput, Error> {
        self.0.call(&operations::GMEDIC_BRANCH_OFFICES_LIST, input).await
    }
}

/// `gmedic.connections`
#[derive(Debug, Clone, Copy)]
pub struct GmedicConnectionsApi<'a>(&'a Client);

impl<'a> GmedicConnectionsApi<'a> {
    /// Remove a provider connection
    ///
    /// Remove the provider-owned connection before Account retires its sealed credentials. Login cooldowns remain durable.
    pub async fn delete(&self, input: &types::GmedicConnectionsDeleteInput) -> Result<types::GmedicConnectionsDeleteOutput, Error> {
        self.0.call(&operations::GMEDIC_CONNECTIONS_DELETE, input).await
    }

    /// Get a provider connection
    ///
    /// One connection owned by the verified organization. Never a credential or a provider request.
    pub async fn get(&self, input: &types::GmedicConnectionsGetInput) -> Result<types::GmedicConnectionsGetOutput, Error> {
        self.0.call(&operations::GMEDIC_CONNECTIONS_GET, input).await
    }

    /// List provider connections
    ///
    /// The organization’s Account connections, without reading credentials or contacting the provider.
    pub async fn list(&self, input: &types::GmedicConnectionsListInput) -> Result<types::GmedicConnectionsListOutput, Error> {
        self.0.call(&operations::GMEDIC_CONNECTIONS_LIST, input).await
    }

    /// Verify a provider connection
    ///
    /// Verify these credentials once before Account seals them. A rejected verification changes no active credential. The durable organization-wide budget prevents replacement connections from bypassing login protection.
    pub async fn verify(&self, input: &types::GmedicConnectionsVerifyInput) -> Result<types::GmedicConnectionsVerifyOutput, Error> {
        self.0.call(&operations::GMEDIC_CONNECTIONS_VERIFY, input).await
    }
}

/// `gmedic.dictionaryEntries`
#[derive(Debug, Clone, Copy)]
pub struct GmedicDictionaryEntriesApi<'a>(&'a Client);

impl<'a> GmedicDictionaryEntriesApi<'a> {
    /// List dictionary entries
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicDictionaryEntriesListInput) -> Result<types::GmedicDictionaryEntriesListOutput, Error> {
        self.0.call(&operations::GMEDIC_DICTIONARY_ENTRIES_LIST, input).await
    }
}

/// `gmedic.healthPartners`
#[derive(Debug, Clone, Copy)]
pub struct GmedicHealthPartnersApi<'a>(&'a Client);

impl<'a> GmedicHealthPartnersApi<'a> {
    /// List health partners
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicHealthPartnersListInput) -> Result<types::GmedicHealthPartnersListOutput, Error> {
        self.0.call(&operations::GMEDIC_HEALTH_PARTNERS_LIST, input).await
    }
}

/// `gmedic.healthPromotionEntities`
#[derive(Debug, Clone, Copy)]
pub struct GmedicHealthPromotionEntitiesApi<'a>(&'a Client);

impl<'a> GmedicHealthPromotionEntitiesApi<'a> {
    /// List health promotion entities
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicHealthPromotionEntitiesListInput) -> Result<types::GmedicHealthPromotionEntitiesListOutput, Error> {
        self.0.call(&operations::GMEDIC_HEALTH_PROMOTION_ENTITIES_LIST, input).await
    }
}

/// `gmedic.laboralRiskManagers`
#[derive(Debug, Clone, Copy)]
pub struct GmedicLaboralRiskManagersApi<'a>(&'a Client);

impl<'a> GmedicLaboralRiskManagersApi<'a> {
    /// List occupational risk managers
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicLaboralRiskManagersListInput) -> Result<types::GmedicLaboralRiskManagersListOutput, Error> {
        self.0.call(&operations::GMEDIC_LABORAL_RISK_MANAGERS_LIST, input).await
    }
}

/// `gmedic.occupations`
#[derive(Debug, Clone, Copy)]
pub struct GmedicOccupationsApi<'a>(&'a Client);

impl<'a> GmedicOccupationsApi<'a> {
    /// List occupations
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicOccupationsListInput) -> Result<types::GmedicOccupationsListOutput, Error> {
        self.0.call(&operations::GMEDIC_OCCUPATIONS_LIST, input).await
    }
}

/// `gmedic.patientNotes`
#[derive(Debug, Clone, Copy)]
pub struct GmedicPatientNotesApi<'a>(&'a Client);

impl<'a> GmedicPatientNotesApi<'a> {
    /// Create patient note
    ///
    /// Use the organization gMedic account. No automatic write retries; reconcile an unknown outcome before repeating.
    pub async fn create(&self, input: &types::GmedicPatientNotesCreateInput) -> Result<types::GmedicPatientNotesCreateOutput, Error> {
        self.0.call(&operations::GMEDIC_PATIENT_NOTES_CREATE, input).await
    }

    /// List patient notes
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicPatientNotesListInput) -> Result<types::GmedicPatientNotesListOutput, Error> {
        self.0.call(&operations::GMEDIC_PATIENT_NOTES_LIST, input).await
    }

    /// Update patient note
    ///
    /// Use the organization gMedic account. No automatic write retries; reconcile an unknown outcome before repeating.
    pub async fn update(&self, input: &types::GmedicPatientNotesUpdateInput) -> Result<types::GmedicPatientNotesUpdateOutput, Error> {
        self.0.call(&operations::GMEDIC_PATIENT_NOTES_UPDATE, input).await
    }
}

/// `gmedic.patients`
#[derive(Debug, Clone, Copy)]
pub struct GmedicPatientsApi<'a>(&'a Client);

impl<'a> GmedicPatientsApi<'a> {
    /// Create patient
    ///
    /// Use the organization gMedic account. No automatic write retries; reconcile an unknown outcome before repeating.
    pub async fn create(&self, input: &types::GmedicPatientsCreateInput) -> Result<types::GmedicPatientsCreateOutput, Error> {
        self.0.call(&operations::GMEDIC_PATIENTS_CREATE, input).await
    }

    /// List patients
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results. Find a patient by identity document with patients.lookup.
    pub async fn list(&self, input: &types::GmedicPatientsListInput) -> Result<types::GmedicPatientsListOutput, Error> {
        self.0.call(&operations::GMEDIC_PATIENTS_LIST, input).await
    }

    /// Look up patients by identity document
    ///
    /// Use the organization gMedic account. Finds the patient with this exact document type and number. Read-only request; an empty page means no such patient, and provider failures are never empty results.
    pub async fn lookup(&self, input: &types::GmedicPatientsLookupInput) -> Result<types::GmedicPatientsLookupOutput, Error> {
        self.0.call(&operations::GMEDIC_PATIENTS_LOOKUP, input).await
    }

    /// Update patient
    ///
    /// Use the organization gMedic account. No automatic write retries; reconcile an unknown outcome before repeating.
    pub async fn update(&self, input: &types::GmedicPatientsUpdateInput) -> Result<types::GmedicPatientsUpdateOutput, Error> {
        self.0.call(&operations::GMEDIC_PATIENTS_UPDATE, input).await
    }
}

/// `gmedic.services`
#[derive(Debug, Clone, Copy)]
pub struct GmedicServicesApi<'a>(&'a Client);

impl<'a> GmedicServicesApi<'a> {
    /// List services
    ///
    /// For voice/AI agents set agentMode=true: up to 10 matches, each with summary, a concise Spanish line with its exact id and code to read to the caller, from a complete catalog cache of at most 60 seconds. Use q (name/code, accent-insensitive), active and kind filters; use a short distinctive term and limit 5 for a caller question. Filtered searches inspect all provider pages before returning matching entries. Incomplete scans fail explicitly. Without filters, returns one catalog page. In every mode, follow pageInfo.nextCursor while hasNextPage is true before concluding an entry is absent. An empty completed search means no matching catalog entry, not that the clinic cannot perform the procedure. A generic procedure or missing price does not establish the requested service or tariff, and an active entry does not confirm that the clinic offers it or its requirements, coverage or appointments: verify them with information from the clinic.
    pub async fn list(&self, input: &types::GmedicServicesListInput) -> Result<types::GmedicServicesListOutput, Error> {
        self.0.call(&operations::GMEDIC_SERVICES_LIST, input).await
    }
}

/// `gmedic.sessionPackages`
#[derive(Debug, Clone, Copy)]
pub struct GmedicSessionPackagesApi<'a>(&'a Client);

impl<'a> GmedicSessionPackagesApi<'a> {
    /// Create session package
    ///
    /// Use the organization gMedic account. No automatic write retries; reconcile an unknown outcome before repeating.
    pub async fn create(&self, input: &types::GmedicSessionPackagesCreateInput) -> Result<types::GmedicSessionPackagesCreateOutput, Error> {
        self.0.call(&operations::GMEDIC_SESSION_PACKAGES_CREATE, input).await
    }

    /// List session packages
    ///
    /// Use the organization gMedic account. Read-only request; provider failures are never empty results.
    pub async fn list(&self, input: &types::GmedicSessionPackagesListInput) -> Result<types::GmedicSessionPackagesListOutput, Error> {
        self.0.call(&operations::GMEDIC_SESSION_PACKAGES_LIST, input).await
    }
}

/// `gmedic`
#[derive(Debug, Clone, Copy)]
pub struct GmedicApi<'a>(&'a Client);

impl<'a> GmedicApi<'a> {
    pub fn agenda_services(&self) -> GmedicAgendaServicesApi<'a> {
        GmedicAgendaServicesApi(self.0)
    }

    pub fn agendas(&self) -> GmedicAgendasApi<'a> {
        GmedicAgendasApi(self.0)
    }

    pub fn appointments(&self) -> GmedicAppointmentsApi<'a> {
        GmedicAppointmentsApi(self.0)
    }

    pub fn availability(&self) -> GmedicAvailabilityApi<'a> {
        GmedicAvailabilityApi(self.0)
    }

    pub fn available_days(&self) -> GmedicAvailableDaysApi<'a> {
        GmedicAvailableDaysApi(self.0)
    }

    pub fn branch_offices(&self) -> GmedicBranchOfficesApi<'a> {
        GmedicBranchOfficesApi(self.0)
    }

    pub fn connections(&self) -> GmedicConnectionsApi<'a> {
        GmedicConnectionsApi(self.0)
    }

    pub fn dictionary_entries(&self) -> GmedicDictionaryEntriesApi<'a> {
        GmedicDictionaryEntriesApi(self.0)
    }

    pub fn health_partners(&self) -> GmedicHealthPartnersApi<'a> {
        GmedicHealthPartnersApi(self.0)
    }

    pub fn health_promotion_entities(&self) -> GmedicHealthPromotionEntitiesApi<'a> {
        GmedicHealthPromotionEntitiesApi(self.0)
    }

    pub fn laboral_risk_managers(&self) -> GmedicLaboralRiskManagersApi<'a> {
        GmedicLaboralRiskManagersApi(self.0)
    }

    pub fn occupations(&self) -> GmedicOccupationsApi<'a> {
        GmedicOccupationsApi(self.0)
    }

    pub fn patient_notes(&self) -> GmedicPatientNotesApi<'a> {
        GmedicPatientNotesApi(self.0)
    }

    pub fn patients(&self) -> GmedicPatientsApi<'a> {
        GmedicPatientsApi(self.0)
    }

    pub fn services(&self) -> GmedicServicesApi<'a> {
        GmedicServicesApi(self.0)
    }

    pub fn session_packages(&self) -> GmedicSessionPackagesApi<'a> {
        GmedicSessionPackagesApi(self.0)
    }
}

/// `nueva-eps.affiliations`
#[derive(Debug, Clone, Copy)]
pub struct NuevaEpsAffiliationsApi<'a>(&'a Client);

impl<'a> NuevaEpsAffiliationsApi<'a> {
    /// Look up member affiliation
    ///
    /// Read member affiliation and primary-provider warnings. Default: structured JSON. Set agentMode=true for concise Spanish text; fields selects top-level output properties. Request member explicitly for additional personal data and timings for measured durations. A provider failure is never an absent member; active affiliation never authorizes a procedure or confirms a clinic contract. No clinical writes are performed.
    pub async fn lookup(&self, input: &types::NuevaEpsAffiliationsLookupInput) -> Result<types::NuevaEpsAffiliationsLookupOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_AFFILIATIONS_LOOKUP, input).await
    }
}

/// `nueva-eps.connections`
#[derive(Debug, Clone, Copy)]
pub struct NuevaEpsConnectionsApi<'a>(&'a Client);

impl<'a> NuevaEpsConnectionsApi<'a> {
    /// Remove a provider connection
    ///
    /// Remove the provider-owned connection before Account retires its sealed credentials. Login cooldowns remain durable.
    pub async fn delete(&self, input: &types::NuevaEpsConnectionsDeleteInput) -> Result<types::NuevaEpsConnectionsDeleteOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_CONNECTIONS_DELETE, input).await
    }

    /// Get a provider connection
    ///
    /// One connection owned by the verified organization. Never a credential or a provider request.
    pub async fn get(&self, input: &types::NuevaEpsConnectionsGetInput) -> Result<types::NuevaEpsConnectionsGetOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_CONNECTIONS_GET, input).await
    }

    /// List provider connections
    ///
    /// The organization’s Account connections, without reading credentials or contacting the provider.
    pub async fn list(&self, input: &types::NuevaEpsConnectionsListInput) -> Result<types::NuevaEpsConnectionsListOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_CONNECTIONS_LIST, input).await
    }

    /// Verify a provider connection
    ///
    /// Verify these credentials once before Account seals them. A rejected verification changes no active credential. The durable organization-wide budget prevents replacement connections from bypassing login protection.
    pub async fn verify(&self, input: &types::NuevaEpsConnectionsVerifyInput) -> Result<types::NuevaEpsConnectionsVerifyOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_CONNECTIONS_VERIFY, input).await
    }
}

/// `nueva-eps`
#[derive(Debug, Clone, Copy)]
pub struct NuevaEpsApi<'a>(&'a Client);

impl<'a> NuevaEpsApi<'a> {
    pub fn affiliations(&self) -> NuevaEpsAffiliationsApi<'a> {
        NuevaEpsAffiliationsApi(self.0)
    }

    pub fn connections(&self) -> NuevaEpsConnectionsApi<'a> {
        NuevaEpsConnectionsApi(self.0)
    }
}

/// `orsovision.agencies`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionAgenciesApi<'a>(&'a Client);

impl<'a> OrsovisionAgenciesApi<'a> {
    /// List the chain's locations
    ///
    /// The chain's locations (agencies), in ascending code order: every other call names one by its code, because each location has its own agreements, services and agendas. Ask the person which location suits them; never pick one for them. Contains no patient data and performs no clinical login. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn list(&self, input: &types::OrsovisionAgenciesListInput) -> Result<types::OrsovisionAgenciesListOutput, Error> {
        self.0.call(&operations::ORSOVISION_AGENCIES_LIST, input).await
    }
}

/// `orsovision.appointments`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionAppointmentsApi<'a>(&'a Client);

impl<'a> OrsovisionAppointmentsApi<'a> {
    /// Cancel an appointment
    ///
    /// Cancel one exact future appointment, only after the person explicitly confirmed it, with the cancellation reference from a fresh patient lookup. Never repeat a cancellation whose outcome is unknown. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn cancel(&self, input: &types::OrsovisionAppointmentsCancelInput) -> Result<types::OrsovisionAppointmentsCancelOutput, Error> {
        self.0.call(&operations::ORSOVISION_APPOINTMENTS_CANCEL, input).await
    }

    /// Book an appointment
    ///
    /// Book one slot orsovision.availability.lookup returned, only after the person explicitly confirmed that exact appointment. Pass its reference unchanged. Never repeat a booking whose outcome is unknown. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn create(&self, input: &types::OrsovisionAppointmentsCreateInput) -> Result<types::OrsovisionAppointmentsCreateOutput, Error> {
        self.0.call(&operations::ORSOVISION_APPOINTMENTS_CREATE, input).await
    }
}

/// `orsovision.availability`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionAvailabilityApi<'a>(&'a Client);

impl<'a> OrsovisionAvailabilityApi<'a> {
    /// Find available appointments
    ///
    /// Real available appointments for a service under a contract, one slot per item in chronological order, each with its professional and the booking reference (token) orsovision.appointments.create takes. limit is how many slots to offer (three in a conversation); for more, call again with the same arguments and pageInfo.nextCursor. window: requested; extended, the dates asked for had none; alternative, the time preference had none (say those first). A completed search without slots is TOOL_REFUSED (NO_AVAILABILITY); never search again without the person's filters. This read never books anything. A refusal is TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead; it is never an outage. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn lookup(&self, input: &types::OrsovisionAvailabilityLookupInput) -> Result<types::OrsovisionAvailabilityLookupOutput, Error> {
        self.0.call(&operations::ORSOVISION_AVAILABILITY_LOOKUP, input).await
    }
}

/// `orsovision.clinic`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionClinicApi<'a>(&'a Client);

impl<'a> OrsovisionClinicApi<'a> {
    /// Get the clinic
    ///
    /// The clinic of the connection a call names, or of the organization's only one: its name, its SIIPS address, its timezone (every date and time the clinical operations answer is local to it) and the version of its tool definitions (orsovision.tools.list). An Agent reads it first in a conversation, to know the clinic and its time. A connection of another clinic is CLINIC_NOT_SERVED; another organization's is CONNECTION_NOT_FOUND. Contains no patient data and performs no clinical login.
    pub async fn get(&self, input: &types::OrsovisionClinicGetInput) -> Result<types::OrsovisionClinicGetOutput, Error> {
        self.0.call(&operations::ORSOVISION_CLINIC_GET, input).await
    }
}

/// `orsovision.contracts`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionContractsApi<'a>(&'a Client);

impl<'a> OrsovisionContractsApi<'a> {
    /// List the clinic's contracts
    ///
    /// The chain's current agreements (plans and payers) at one of its locations, in ascending code order. A name that matches no agreement is TOOL_REFUSED (CONTRACT_NOT_FOUND) with the closest names in details.didYouMean, to offer the person. Preserve the returned identifiers; a failed lookup never proves a patient lacks coverage. A refusal is TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead; it is never an outage. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn list(&self, input: &types::OrsovisionContractsListInput) -> Result<types::OrsovisionContractsListOutput, Error> {
        self.0.call(&operations::ORSOVISION_CONTRACTS_LIST, input).await
    }
}

/// `orsovision.patients`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionPatientsApi<'a>(&'a Client);

impl<'a> OrsovisionPatientsApi<'a> {
    /// Correct a patient's details
    ///
    /// Update only the patient fields the person explicitly corrected and confirmed; omit unchanged fields. Never repeat an update whose outcome is unknown. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn correct(&self, input: &types::OrsovisionPatientsCorrectInput) -> Result<types::OrsovisionPatientsCorrectOutput, Error> {
        self.0.call(&operations::ORSOVISION_PATIENTS_CORRECT, input).await
    }

    /// Register a patient
    ///
    /// Register a patient only after a lookup confirmed it is not found and the person explicitly confirmed the required details. Never repeat a registration whose outcome is unknown. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn create(&self, input: &types::OrsovisionPatientsCreateInput) -> Result<types::OrsovisionPatientsCreateOutput, Error> {
        self.0.call(&operations::ORSOVISION_PATIENTS_CREATE, input).await
    }

    /// Look up a patient
    ///
    /// The patient's profile, recent and upcoming appointments (each upcoming one with its cancellation reference), by exact document number, after the clinic's identity and consent checks. found:false is an expected answer: confirm the document and offer to register the patient. Reuse what the patient already confirmed and avoid unrelated patient data. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn lookup(&self, input: &types::OrsovisionPatientsLookupInput) -> Result<types::OrsovisionPatientsLookupOutput, Error> {
        self.0.call(&operations::ORSOVISION_PATIENTS_LOOKUP, input).await
    }
}

/// `orsovision.services`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionServicesApi<'a>(&'a Client);

impl<'a> OrsovisionServicesApi<'a> {
    /// List a contract's services
    ///
    /// The services of one of the chain's agreements (code) at one of its locations, in ascending code order; with q, only those whose code or name contains it. Catalog items that cannot be booked are not appointment availability. A refusal is TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead; it is never an outage. Orsovision's tool pack sets this tool's rules, parameters and accepted values (orsovision.tools.list answers them), and every call is checked against them.
    pub async fn list(&self, input: &types::OrsovisionServicesListInput) -> Result<types::OrsovisionServicesListOutput, Error> {
        self.0.call(&operations::ORSOVISION_SERVICES_LIST, input).await
    }
}

/// `orsovision.tools`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionToolsApi<'a>(&'a Client);

impl<'a> OrsovisionToolsApi<'a> {
    /// List the clinic's tools
    ///
    /// Every tool Orsovision's agents may call, in ascending name order, with the clinic's exact parameters and instructions, each naming the operation that runs it; a list's tool declares the page fields too. The clinic itself, its timezone and the version of these definitions are orsovision.clinic.get's. Contains no patient data and performs no clinical login.
    pub async fn list(&self, input: &types::OrsovisionToolsListInput) -> Result<types::OrsovisionToolsListOutput, Error> {
        self.0.call(&operations::ORSOVISION_TOOLS_LIST, input).await
    }
}

/// `orsovision`
#[derive(Debug, Clone, Copy)]
pub struct OrsovisionApi<'a>(&'a Client);

impl<'a> OrsovisionApi<'a> {
    pub fn agencies(&self) -> OrsovisionAgenciesApi<'a> {
        OrsovisionAgenciesApi(self.0)
    }

    pub fn appointments(&self) -> OrsovisionAppointmentsApi<'a> {
        OrsovisionAppointmentsApi(self.0)
    }

    pub fn availability(&self) -> OrsovisionAvailabilityApi<'a> {
        OrsovisionAvailabilityApi(self.0)
    }

    pub fn clinic(&self) -> OrsovisionClinicApi<'a> {
        OrsovisionClinicApi(self.0)
    }

    pub fn contracts(&self) -> OrsovisionContractsApi<'a> {
        OrsovisionContractsApi(self.0)
    }

    pub fn patients(&self) -> OrsovisionPatientsApi<'a> {
        OrsovisionPatientsApi(self.0)
    }

    pub fn services(&self) -> OrsovisionServicesApi<'a> {
        OrsovisionServicesApi(self.0)
    }

    pub fn tools(&self) -> OrsovisionToolsApi<'a> {
        OrsovisionToolsApi(self.0)
    }
}

/// `raiz.agents`
#[derive(Debug, Clone, Copy)]
pub struct RaizAgentsApi<'a>(&'a Client);

impl<'a> RaizAgentsApi<'a> {
    /// Create an Agent
    ///
    /// Create an Agent that talks to the organization's contacts. Choose `modality` (`voice`: phone calls; `text`: WhatsApp and web chat) and a `name`; then give it `instructions` (role, goal, limits), and optionally a `firstMessage`, `language`, `timeZone`, `settings`, Knowledge (`knowledgeDocumentIds` of ready documents from `list_knowledge_documents`) and `integrationTools` (connection + tool pairs of the organization's connections in Account, checked live: INTEGRATION_TOOL_UNAVAILABLE), `procedures` (each a trigger plus free-form Markdown or structured steps, with inline references to the Agent's tools, Knowledge and other procedures: AGENT_PROCEDURES_INVALID says where one is wrong), and for a text Agent `integrationProcedures` (connection + procedure pairs a connection publishes, checked live: INTEGRATION_PROCEDURE_UNAVAILABLE). Next: assign it to a channel with `update_channel`; a voice Agent is published in the background, so check `synced` with `get_agent`. Read the `create-agent` skill (`get_skill`) first.
    pub async fn create(&self, input: &types::RaizAgentsCreateInput) -> Result<types::RaizAgentsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_AGENTS_CREATE, input).await
    }

    /// Get an Agent
    ///
    /// Read one Agent's complete setup: instructions, first message, language, time zone, settings, integration tools (each with its connection and effect), Knowledge document ids, procedures (trigger plus free-form instructions or structured steps) and connection procedures, and channels. `ready` says whether it can hold conversations; for a voice Agent, `synced` says whether its phone line already runs this version, and `syncErrorCode` why not. Read it before `update_agent` and send its `version` as `expectedVersion`.
    pub async fn get(&self, input: &types::RaizAgentsGetInput) -> Result<types::RaizAgentsGetOutput, Error> {
        self.0.call(&operations::RAIZ_AGENTS_GET, input).await
    }

    /// List Agents
    ///
    /// Find the organization's Agents (the Dashboard calls them Workers), newest first, to get the `agentId` you need before reading or changing one. Filter by `status` (`active`, `paused`), `modality` (`voice` for calls, `text` for WhatsApp and web chat) or `q` (name). Each item shows whether it is `ready` to hold conversations and, for voice Agents, `synced`. Next: `get_agent` for its full setup, or `list_conversations` with `agentId` for what it talked about.
    pub async fn list(&self, input: &types::RaizAgentsListInput) -> Result<types::RaizAgentsListOutput, Error> {
        self.0.call(&operations::RAIZ_AGENTS_LIST, input).await
    }

    /// Update an Agent
    ///
    /// Change an Agent: instructions, first message, name, language, time zone, settings, integration tools, Knowledge, procedures, connection procedures or status. Omitted fields stay and `null` clears one; `settings`, `integrationTools`, `integrationProcedures`, `procedures` and `knowledgeDocumentIds` replace the whole value when given (keep each existing procedure's `id`), so read the Agent with `get_agent` first and send its `version` as `expectedVersion` (VERSION_CONFLICT: read it again). `status: paused` stops it from answering and calling. `modality` changes only while it has no channels and no published voice line (AGENT_MODALITY_LOCKED). A voice Agent republishes in the background: check `synced`.
    pub async fn update(&self, input: &types::RaizAgentsUpdateInput) -> Result<types::RaizAgentsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_AGENTS_UPDATE, input).await
    }
}

/// `raiz.bulletins`
#[derive(Debug, Clone, Copy)]
pub struct RaizBulletinsApi<'a>(&'a Client);

impl<'a> RaizBulletinsApi<'a> {
    /// Post a bulletin
    ///
    /// Post a bulletin: something happening now that the organization's Agents must apply on calls, WhatsApp and web chat while it lasts (for example: there is no nutrition service; no appointments at the North site for four days; closed on Monday the 26th; a promotion). Write `text` as you would tell the Agents, including what they should offer instead. Omit `agentId` for every Agent, or name one Agent. Agents apply it from their next reply or call once `startsAt` (default now) is reached, until `endsAt` (omit it for no end). Resolve phrases such as 'for four days' or 'until Friday' in the organization's time zone (`get_organization`), ending whole days at local midnight. No Agent reads a bulletin before it starts: for Agents to know ahead (so they book nothing on a closed day), start it now and write the dates in the text. Fails with BULLETIN_PERIOD_INVALID when it would end before it starts or is already over, and BULLETIN_LIMIT_REACHED at 30 bulletins in force or scheduled.
    pub async fn create(&self, input: &types::RaizBulletinsCreateInput) -> Result<types::RaizBulletinsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_BULLETINS_CREATE, input).await
    }

    /// Delete a bulletin
    ///
    /// Delete a bulletin that was a mistake or is no longer wanted: Agents stop applying it from their next reply or call. The history keeps who deleted it and when. To keep it listed as ended instead, set its `endsAt` to now with `update_bulletin`.
    pub async fn delete(&self, input: &types::RaizBulletinsDeleteInput) -> Result<types::RaizBulletinsDeleteOutput, Error> {
        self.0.call(&operations::RAIZ_BULLETINS_DELETE, input).await
    }

    /// Get a bulletin
    ///
    /// Read one bulletin: its text, the Agent that applies it (null for every Agent), its period and its status.
    pub async fn get(&self, input: &types::RaizBulletinsGetInput) -> Result<types::RaizBulletinsGetOutput, Error> {
        self.0.call(&operations::RAIZ_BULLETINS_GET, input).await
    }

    /// List bulletins
    ///
    /// List the organization's bulletins (Settings → Bulletins in the Dashboard; people may call them novedades, avisos or eventos): what is happening now that its Agents apply while it lasts, such as a service that is not available, days without appointments at a site, a closed day or a promotion. Default: those in force (`active`) and those not started yet (`scheduled`); `status=ended` lists ended ones. `agentId` lists what one Agent applies (its own and those for every Agent); `q` searches the text. Use it to find the bulletin a person means before changing or deleting it.
    pub async fn list(&self, input: &types::RaizBulletinsListInput) -> Result<types::RaizBulletinsListOutput, Error> {
        self.0.call(&operations::RAIZ_BULLETINS_LIST, input).await
    }

    /// Update a bulletin
    ///
    /// Change a bulletin: its text, its Agent (null for every Agent), its start or its end. Omitted fields stay. To end one now, set `endsAt` to the current instant; a scheduled one is deleted instead. Agents apply the change from their next reply or call.
    pub async fn update(&self, input: &types::RaizBulletinsUpdateInput) -> Result<types::RaizBulletinsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_BULLETINS_UPDATE, input).await
    }
}

/// `raiz.campaigns`
#[derive(Debug, Clone, Copy)]
pub struct RaizCampaignsApi<'a>(&'a Client);

impl<'a> RaizCampaignsApi<'a> {
    /// Create a call campaign
    ///
    /// Prepare an outbound call campaign for a voice Agent with up to 10000 recipients, each a phone `address` with an optional `contactId`, `variables`, `firstMessage` and `instructions`. Nobody is called until it is started. Each recipient's contact is the one it names, or the owner of its number (created when none exists). Calls happen only inside `contactHours` (in `timeZone`, the organization's by default) and between `scheduledStartAt` and `scheduledEndAt`; each recipient gets up to `maxAttempts` calls, retried `retryAfterMinutes` later after the results in `retryOn` (`no_answer`, `failure`). Starting it (REST `POST /v1/campaigns/{campaignId}/start`) is not an AI tool. Read the `run-a-campaign` skill.
    pub async fn create(&self, input: &types::RaizCampaignsCreateInput) -> Result<types::RaizCampaignsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_CREATE, input).await
    }

    /// Get a campaign
    ///
    /// Read one campaign: Agent, schedule, contact hours, retries and progress. `expand=recipients` adds its recipients in the order they were added, each with attempts, next attempt, error and latest call; continue with `recipientsCursor` (`recipientsDirection` reads the other way). Read it before `update_campaign` and send `expectedVersion`.
    pub async fn get(&self, input: &types::RaizCampaignsGetInput) -> Result<types::RaizCampaignsGetOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_GET, input).await
    }

    /// List campaigns
    ///
    /// List the organization's call campaigns, newest first, each with its progress (recipients pending, in progress, completed, failed and succeeded). Filter by `status` or `agentId`. A campaign's calls: `list_conversations` with `campaignId`; its numbers by day: `get_report` with `name: campaign_progress`.
    pub async fn list(&self, input: &types::RaizCampaignsListInput) -> Result<types::RaizCampaignsListOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_LIST, input).await
    }

    /// Update a campaign
    ///
    /// Change a campaign. Omitted fields stay. `paused: true` pauses a started campaign (no new call starts; calls in progress finish) and `paused: false` resumes it, with the same checks as starting. Before the start, `recipients` is the complete set (matched by `address`); once started it only adds, so send a long list in batches. The Agent changes only before the start, and a completed campaign accepts only its name and metadata.
    pub async fn update(&self, input: &types::RaizCampaignsUpdateInput) -> Result<types::RaizCampaignsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_UPDATE, input).await
    }
}

/// `raiz.changes`
#[derive(Debug, Clone, Copy)]
pub struct RaizChangesApi<'a>(&'a Client);

impl<'a> RaizChangesApi<'a> {
    /// List changes (audit log)
    ///
    /// See who created, changed or deleted what, when and why, newest first, with each changed field's value before and after. Filter by resource, kind of resource, contact (including the duplicates it absorbed), author and time. Use it to answer questions such as who changed an Agent's instructions and when. Values can include contacts' personal data.
    pub async fn list(&self, input: &types::RaizChangesListInput) -> Result<types::RaizChangesListOutput, Error> {
        self.0.call(&operations::RAIZ_CHANGES_LIST, input).await
    }
}

/// `raiz.channels`
#[derive(Debug, Clone, Copy)]
pub struct RaizChannelsApi<'a>(&'a Client);

impl<'a> RaizChannelsApi<'a> {
    /// Add a channel
    ///
    /// Add a channel. `web` creates a website chat or form (the widget) from its complete configuration (`widget`: mode, texts, fields, colors and fonts, allowed origins, `publicLink` for its own hosted page); it is the one kind an AI tool creates. Create it with `inboundEnabled: false` to review it before publishing, then publish or pause it with `update_channel`; `publication` says whether visitors can reach it and where. `whatsapp` creates a pending channel and a setup link (`setup.url`) where the business connects its WhatsApp number; call `verify_channel` once that is done. `voice` connects a platform phone number (`platform_number`) or the organization's own carrier over SIP (`sip_trunk`) at once (SIP credentials are passed on once and never stored). A person adds WhatsApp and voice channels in Settings → Channels or with the REST API: over MCP they fail with CHANNEL_CREATION_IN_APP. With an Agent, traffic is on by default. Not idempotent: after an unknown outcome, `list_channels` before trying again.
    pub async fn create(&self, input: &types::RaizChannelsCreateInput) -> Result<types::RaizChannelsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_CHANNELS_CREATE, input).await
    }

    /// Delete a channel
    ///
    /// Remove a channel; its conversations remain. A phone number stops receiving and placing calls (the number stays in the platform account, where Arbol staff can release it); a WhatsApp number stops sending messages to Raíz. To pause traffic instead, switch it off with `update_channel`. Not an AI tool.
    pub async fn delete(&self, input: &types::RaizChannelsDeleteInput) -> Result<types::RaizChannelsDeleteOutput, Error> {
        self.0.call(&operations::RAIZ_CHANNELS_DELETE, input).await
    }

    /// Get a channel
    ///
    /// Read one channel: its optional Agent, `inboundEnabled` and `outboundEnabled`, status and setup state; a web channel includes its widget configuration, and `expand=profile` adds the WhatsApp business profile, read live from WhatsApp. Inbound automation requires an Agent; WhatsApp outbound notifications do not. Traffic must be enabled and the channel active. `aiCoveragePercent` assigns 0–100% of new inbound WhatsApp conversations to AI; zero records human-managed conversations. Existing episodes keep their assignment.
    pub async fn get(&self, input: &types::RaizChannelsGetInput) -> Result<types::RaizChannelsGetOutput, Error> {
        self.0.call(&operations::RAIZ_CHANNELS_GET, input).await
    }

    /// List channels
    ///
    /// List the organization's WhatsApp numbers, phone numbers and web chat widgets, newest first. Filter by `agentId` (an Agent's channels), `channelType` (`whatsapp`, `voice`, `web`) or `status` (`pending`, `active`, `degraded`). Each shows its optional Agent, traffic switches and whether it is `ready`. WhatsApp notifications can send without an Agent. Next: `get_channel` for details, or `update_channel` to configure traffic and assignment.
    pub async fn list(&self, input: &types::RaizChannelsListInput) -> Result<types::RaizChannelsListOutput, Error> {
        self.0.call(&operations::RAIZ_CHANNELS_LIST, input).await
    }

    /// Renew a WhatsApp setup link
    ///
    /// Issue a new setup link for a pending WhatsApp channel, returned in `setup`, when the previous one was lost or must go to someone else; the previous link stops working. The setup is checked first: if the business already connected its number, the channel is completed as `verify_channel` does and no link is returned. A channel that is not a pending WhatsApp number fails with INVALID_STATE or CHANNEL_TYPE_MISMATCH. Not an AI tool.
    pub async fn renew_setup(&self, input: &types::RaizChannelsRenewSetupInput) -> Result<types::RaizChannelsRenewSetupOutput, Error> {
        self.0.call(&operations::RAIZ_CHANNELS_RENEW_SETUP, input).await
    }

    /// Update a channel
    ///
    /// Assign the Agent that answers on a channel, switch its traffic, or change what people see. `agentId` assigns an Agent whose modality fits the channel (`null` stops inbound automation and preserves WhatsApp outbound traffic). Inbound traffic and outbound traffic on voice or web channels require an Agent; WhatsApp outbound notifications do not. WhatsApp channels accept `profile` (the business profile people see in WhatsApp); web channels accept `widget` (the complete new widget configuration) and `widgetLogo` (PNG, JPEG or WebP up to 256 KiB; `null` removes it). Omitted fields stay.
    pub async fn update(&self, input: &types::RaizChannelsUpdateInput) -> Result<types::RaizChannelsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_CHANNELS_UPDATE, input).await
    }

    /// Verify a channel
    ///
    /// Check a channel with its provider and record the result. Call it after the business finishes the WhatsApp setup link: the pending channel becomes `active`. An expired setup link is replaced and returned in `setup`, ready to send to the person again. Fails with CHANNEL_ENDPOINT_IN_USE when another channel already holds that number; a failed check marks the channel `degraded`.
    pub async fn verify(&self, input: &types::RaizChannelsVerifyInput) -> Result<types::RaizChannelsVerifyOutput, Error> {
        self.0.call(&operations::RAIZ_CHANNELS_VERIFY, input).await
    }
}

/// `raiz.contacts`
#[derive(Debug, Clone, Copy)]
pub struct RaizContactsApi<'a>(&'a Client);

impl<'a> RaizContactsApi<'a> {
    /// Count contacts
    ///
    /// Count the people `list_contacts` returns for the same filters (`q`, `channelType`, `agentId`, `handling`, `lastContactedAfter`), up to 10,000: `capped` then says at least that many match. Use it to answer how many people match instead of paging through the list.
    pub async fn count(&self, input: &types::RaizContactsCountInput) -> Result<types::RaizContactsCountOutput, Error> {
        self.0.call(&operations::RAIZ_CONTACTS_COUNT, input).await
    }

    /// Create a contact
    ///
    /// Add a person with their contact points (phones, WhatsApp, emails) and identity documents. Search with `list_contacts` first to avoid duplicates: it fails with ADDRESS_IN_USE or IDENTIFIER_IN_USE when another contact already has one of them, and nothing is created. For many people at once, use `import_contacts`.
    pub async fn create(&self, input: &types::RaizContactsCreateInput) -> Result<types::RaizContactsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_CONTACTS_CREATE, input).await
    }

    /// Get a contact
    ///
    /// Read one person: names, contact points (phones, WhatsApp, emails), identity documents, summary and current facts. Add `expand=timeline` for what happened to them (conversations, facts, changes, merges), newest first; continue with `timelineCursor` (and `timelineDirection`, `timelineLimit`). The id of a merged duplicate returns the contact that absorbed it. Read it before `update_contact` and send its `version` as `expectedVersion`.
    pub async fn get(&self, input: &types::RaizContactsGetInput) -> Result<types::RaizContactsGetOutput, Error> {
        self.0.call(&operations::RAIZ_CONTACTS_GET, input).await
    }

    /// Import contacts from a CSV
    ///
    /// Add or update many people at once from a CSV, given as text (`csv`) or as a base64 file (`file`, up to 12 MB, about 100,000 rows). Rows matching an existing contact's phone, WhatsApp, email or document update that contact; the rest create new ones. Returns a task at once: follow it with `get_task` (`processedRows` of `totalRows`, then created, updated and rejected counts with the first 100 row errors). `cancel_task` stops it after the current batch; a failed import resumes with `retry_task` within 7 days. Fails with IMPORT_FILE_INVALID when the file is not UTF-8 and IMPORT_FILE_TOO_LARGE above 12 MB. Read the `import-contacts` skill for the columns.
    pub async fn import(&self, input: &types::RaizContactsImportInput) -> Result<types::RaizContactsImportOutput, Error> {
        self.0.call(&operations::RAIZ_CONTACTS_IMPORT, input).await
    }

    /// List contacts
    ///
    /// Find people in the organization's contacts (People in the Dashboard), newest first by default. `q` matches names, channel profile names, usernames, phones, emails, identity documents and contact ids (ignoring accents, case and number formatting); `channelType`, `agentId`, `handling` (`ai`: the AI took part in one of their conversations; `team`: only people answered) and `lastContactedAfter` narrow it; `sort` is `created`, `name` or `last_contacted`. Each person carries their latest conversation; `count_contacts` counts every match. Start here to get a `contactId`; with several matches, ask which person is meant. Next: `get_contact`, or `list_conversations` with `contactId` for their conversations.
    pub async fn list(&self, input: &types::RaizContactsListInput) -> Result<types::RaizContactsListOutput, Error> {
        self.0.call(&operations::RAIZ_CONTACTS_LIST, input).await
    }

    /// Update a contact
    ///
    /// Change a person's details. Omitted fields stay and `null` clears one. `points` and `identifiers`, when given, are the complete new sets: listed items keep their id and verification, missing ones are removed, so read the contact with `get_contact` first and send `expectedVersion`. Names and summaries written here are never overwritten by the AI.
    pub async fn update(&self, input: &types::RaizContactsUpdateInput) -> Result<types::RaizContactsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_CONTACTS_UPDATE, input).await
    }
}

/// `raiz.conversations`
#[derive(Debug, Clone, Copy)]
pub struct RaizConversationsApi<'a>(&'a Client);

impl<'a> RaizConversationsApi<'a> {
    /// Give a conversation back to the AI to continue
    ///
    /// Give a WhatsApp conversation the AI handed to the team back to the AI, which continues the contact's pending request now, though the contact has not written. `note` tells the Agent what changed; the contact never sees it. Only while WhatsApp's 24-hour window is open (WHATSAPP_WINDOW_CLOSED otherwise, and nothing changes), once per handoff (HANDOFF_ALREADY_CONTINUED), and only for a conversation the AI handed off (CONVERSATION_NOT_HANDED_OFF). Not an AI tool: it makes the Agent write to a real person.
    pub async fn r#continue(&self, input: &types::RaizConversationsContinueInput) -> Result<types::RaizConversationsContinueOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_CONTINUE, input).await
    }

    /// Count conversations
    ///
    /// Count the conversations `list_conversations` returns for the same filters (every filter it takes, without `sort`), up to 10,000: `capped` then says at least that many match. Use it to answer how many conversations match instead of paging through the list; for volumes per day, Agent or outcome use `get_report`.
    pub async fn count(&self, input: &types::RaizConversationsCountInput) -> Result<types::RaizConversationsCountOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_COUNT, input).await
    }

    /// Delete a conversation
    ///
    /// Delete a closed conversation: it leaves every list and report, its transcript and facts are kept, and the history records who deleted it. Not an AI tool.
    pub async fn delete(&self, input: &types::RaizConversationsDeleteInput) -> Result<types::RaizConversationsDeleteOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_DELETE, input).await
    }

    /// Get a conversation
    ///
    /// Read one conversation with its contact, Agent and channel, who works it (`workers`: each Agent and the team, and who answers now), title, summary, outcome, sentiment and current facts. Add `expand=messages` for the transcript, oldest first, each message with its author, the Agent's tool calls and quoted replies (continue with `messagesCursor`), and `expand=recording` for a short-lived link to the call recording. Staff may also request `expand=diagnostics` for bounded live call-termination evidence; this additionally requires raiz:staff:operate and never changes the conversation. The `review-conversation` skill explains how to read and summarize one.
    pub async fn get(&self, input: &types::RaizConversationsGetInput) -> Result<types::RaizConversationsGetOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_GET, input).await
    }

    /// List conversations
    ///
    /// Find calls, WhatsApp chats and web chats. Filter by `contactId` (as counterpart or subject), `agentId`, `channelId`, `channelType`, `status`, `conversationDirection`, `origin`, `outcome`, `handling` (`ai` or `team`: whether the AI takes part; `escalated`: handed to the team by the AI), `worker` (`ai` or `team`: who answers it now), `campaignId`, `profileId`, `requestedBy` and start time (`startedAfter`, `startedBefore`); `q` searches participant identities (names, profile names, usernames, phones, emails and documents), visible messages, titles, summaries and exact conversation references; `count_conversations` counts every match. Each item already carries its title, summary, outcome, sentiment and workers, often enough to answer. `sort`: `started` (newest first), `last_message` (latest activity first) or `changed`, the feed of status, outcome and analysis changes, oldest first, that systems poll from a saved `pageInfo.nextCursor` (a deleted conversation appears once with `deletedAt`). Next: `get_conversation` for a transcript.
    pub async fn list(&self, input: &types::RaizConversationsListInput) -> Result<types::RaizConversationsListOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_LIST, input).await
    }

    /// Update a conversation
    ///
    /// Change a conversation's title, summary, metadata, contact or subject, or close it (`status: closed`), which starts its analysis. A title or summary written here always wins over the generated one. A call closes by itself (CALL_IN_PROGRESS); a web chat cannot close while the Agent is replying (REPLY_IN_PROGRESS, retry shortly). On WhatsApp, `aiEnabled` activates or pauses AI only for this open conversation and leaves future episodes unchanged; `aiPaused` pauses or resumes the Agent for this person until their open conversation closes. Activating or resuming answers what the contact wrote after the last real answer, the team's or the Agent's (a wait notice is not one).
    pub async fn update(&self, input: &types::RaizConversationsUpdateInput) -> Result<types::RaizConversationsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_UPDATE, input).await
    }
}

/// `raiz.definitions`
#[derive(Debug, Clone, Copy)]
pub struct RaizDefinitionsApi<'a>(&'a Client);

impl<'a> RaizDefinitionsApi<'a> {
    /// Create a data definition
    ///
    /// Define one thing Agents should capture. `kind`: `attribute` (a property of the subject), `assessment` (a judgement about a conversation; yes/no ones feed the `assessment_results_daily` report), `category` (a classification) or `metric` (a measured number with a `unit`). `subject`: `contact` or `conversation`. `valueType`: `text`, `number`, `boolean`, `date` or `option` (with its `options`). Give a unique `key`, a `name`, optional `instructions` telling the Agent how to decide, and the `agentIds` that collect it. Fails with KEY_IN_USE when a draft or active definition already has the key.
    pub async fn create(&self, input: &types::RaizDefinitionsCreateInput) -> Result<types::RaizDefinitionsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_DEFINITIONS_CREATE, input).await
    }

    /// Get a data definition
    ///
    /// Read one definition in any status: kind, subject, value type, options, instructions for the Agent and the Agents that use it. Read it before `update_definition` and send its `version` as `expectedVersion`.
    pub async fn get(&self, input: &types::RaizDefinitionsGetInput) -> Result<types::RaizDefinitionsGetOutput, Error> {
        self.0.call(&operations::RAIZ_DEFINITIONS_GET, input).await
    }

    /// List data definitions
    ///
    /// List what the organization's Agents capture: each definition is one data point (an `attribute`, `category` or `metric` of a contact or conversation, or an `assessment` of a conversation) with its key, value type, options and the Agents that use it. Draft and active ones by default; `status=retired` lists retired ones; filter by `kind` and `subject`. In the Dashboard these are an Agent's Data it collects and Conversation reports, and Settings → Definitions.
    pub async fn list(&self, input: &types::RaizDefinitionsListInput) -> Result<types::RaizDefinitionsListOutput, Error> {
        self.0.call(&operations::RAIZ_DEFINITIONS_LIST, input).await
    }

    /// Update a data definition
    ///
    /// Change a definition: name, instructions, options, Agents, unit or status. Omitted fields stay and `null` clears one; `options` and `agentIds`, when given, are the complete new sets (an option left out is retired, never deleted). Value type and subject cannot change once facts exist (DEFINITION_TYPE_LOCKED), and a retired definition no longer changes. Read it with `get_definition` and send `expectedVersion`.
    pub async fn update(&self, input: &types::RaizDefinitionsUpdateInput) -> Result<types::RaizDefinitionsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_DEFINITIONS_UPDATE, input).await
    }
}

/// `raiz.evalCases`
#[derive(Debug, Clone, Copy)]
pub struct RaizEvalCasesApi<'a>(&'a Client);

impl<'a> RaizEvalCasesApi<'a> {
    /// Record Agent feedback
    ///
    /// Record internal feedback on one completed WhatsApp Agent reply or an Agent response from a completed voice call: what should be preserved or changed, a preferred response, and whether it was incorrect, incomplete or could be simpler. Requires conversation write and Agent read access. Does not contact anyone or change live instructions. Repeating creation returns the existing authored case; edit it with updateEvalCase and expectedVersion.
    pub async fn create(&self, input: &types::RaizEvalCasesCreateInput) -> Result<types::RaizEvalCasesCreateOutput, Error> {
        self.0.call(&operations::RAIZ_EVAL_CASES_CREATE, input).await
    }

    /// List evaluation cases
    ///
    /// The Agents' evaluation cases, newest first: WhatsApp replies or completed voice-call responses with explicit internal team feedback (`authored`) or a following team message (`correction`), and the originating Agent version when recorded (unknown for historical voice calls). Filter by agentId, replyMessageId, source or status (`open`, `kept`, `dismissed`). Requires conversation read authority as well as Agent read authority. Read the conversation with its own operations to judge a case. Not an AI tool.
    pub async fn list(&self, input: &types::RaizEvalCasesListInput) -> Result<types::RaizEvalCasesListOutput, Error> {
        self.0.call(&operations::RAIZ_EVAL_CASES_LIST, input).await
    }

    /// Review an evaluation case
    ///
    /// Review an evaluation case: keep it (`status: kept`) so candidate versions are measured on it, dismiss it, and write in `expectation` what the Agent should have done (a short rubric, no personal data; `null` clears it). Not an AI tool.
    pub async fn update(&self, input: &types::RaizEvalCasesUpdateInput) -> Result<types::RaizEvalCasesUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_EVAL_CASES_UPDATE, input).await
    }
}

/// `raiz.evalRuns`
#[derive(Debug, Clone, Copy)]
pub struct RaizEvalRunsApi<'a>(&'a Client);

impl<'a> RaizEvalRunsApi<'a> {
    /// Replay evaluation cases on a candidate
    ///
    /// With feedbackCaseId, prepare conditional guidance from that kept correction and test it with the Agent’s newest kept cases, always including the source. Do not combine feedbackCaseId with candidate or caseIds. This does not publish. Requires conversation read access. Voice Agents use a spoken-content rehearsal with an independent model (evaluationMode voice_text), not the live voice model, audio, interruptions or telephony. A person must review its applicability to calls before approval. Otherwise measure a candidate version on its kept evaluation cases before publishing it: the WhatsApp Agent rewrites each case's reply with the candidate's `instructions`, `procedures`, `integrationProcedures`, `model` and `reasoningEffort` (omitted ones stay as the live Agent has them), without running any integration tool (each answers with what it answered in the original conversation) and without sending anything; then a judge model compares each new reply with what the team expects. Replays every kept case, or `caseIds`, at most 50. It calls paid models, so it needs `confirmCost: true`. Follow it with `getEvalRun`; with the Agent's `settings.evalGate`, `updateAgent` publishes exactly this content with `evalRunId`. EVAL_CASES_UNAVAILABLE: no kept case to replay. Not an AI tool.
    pub async fn create(&self, input: &types::RaizEvalRunsCreateInput) -> Result<types::RaizEvalRunsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_EVAL_RUNS_CREATE, input).await
    }

    /// Get an evaluation run
    ///
    /// Read an evaluation run: its `status`, how many cases `passed`, `failed` or could not be replayed (`errors`), and each case's result as it is judged: the candidate's reply as the contact would have read it, whether it escalated or handed off, the tools it called and the judge's reasons. Not an AI tool.
    pub async fn get(&self, input: &types::RaizEvalRunsGetInput) -> Result<types::RaizEvalRunsGetOutput, Error> {
        self.0.call(&operations::RAIZ_EVAL_RUNS_GET, input).await
    }
}

/// `raiz.facts`
#[derive(Debug, Clone, Copy)]
pub struct RaizFactsApi<'a>(&'a Client);

impl<'a> RaizFactsApi<'a> {
    /// Record a fact
    ///
    /// Record the value of a definition for a contact or a conversation, or that it has none (`valueStatus: unknown` or `not_applicable`). Only record something actually observed, and say where it came from (`sourceConversationId`, `sourceMessageId`, `reason`). The value follows the definition's type: text, number, boolean, a `YYYY-MM-DD` date or an option key. The newest assertion that was not rejected becomes the current fact; older ones stay as history. Get `definitionId` from `list_definitions`.
    pub async fn create(&self, input: &types::RaizFactsCreateInput) -> Result<types::RaizFactsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_FACTS_CREATE, input).await
    }

    /// Review a fact
    ///
    /// Confirm or reject a recorded fact: `confirmed` vouches for it; `rejected` withdraws it, and the previous assertion becomes current again. The history records who reviewed it.
    pub async fn update(&self, input: &types::RaizFactsUpdateInput) -> Result<types::RaizFactsUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_FACTS_UPDATE, input).await
    }
}

/// `raiz.knowledge`
#[derive(Debug, Clone, Copy)]
pub struct RaizKnowledgeApi<'a>(&'a Client);

impl<'a> RaizKnowledgeApi<'a> {
    /// Add a Knowledge document
    ///
    /// Add a document Agents can consult. `kind: text` takes `text` (plain text or Markdown); `kind: file` takes `file` (base64, up to 8 MiB: PDF, Word, Excel, HTML, CSV, plain text or Markdown); `kind: website` takes an approved HTTPS `sourceUrl` and optional `crawl` bounds and starts a crawl task (`list_tasks` with `kind=knowledge_crawl`). Indexing follows on its own: the document is searchable once `indexStatus` is `ready` (check with `get_knowledge_document`). Then give it to Agents with `update_agent` (`knowledgeDocumentIds`). Read the `manage-knowledge` skill.
    pub async fn create(&self, input: &types::RaizKnowledgeCreateInput) -> Result<types::RaizKnowledgeCreateOutput, Error> {
        self.0.call(&operations::RAIZ_KNOWLEDGE_CREATE, input).await
    }

    /// Get a Knowledge document
    ///
    /// Read one document and its indexing state. Add `expand=content` to read its text as indexed, in Markdown, up to 200,000 characters: use it to show or quote what a document actually says.
    pub async fn get(&self, input: &types::RaizKnowledgeGetInput) -> Result<types::RaizKnowledgeGetOutput, Error> {
        self.0.call(&operations::RAIZ_KNOWLEDGE_GET, input).await
    }

    /// List Knowledge documents
    ///
    /// List the documents in the organization's knowledge base (Settings → Knowledge Base), newest first. Filter by `kind` (`text`, `file`, `website`), `status` and `indexStatus`; `q` matches titles and file names. Only documents with `indexStatus: ready` are searchable. Which Agents use a document is each Agent's `knowledgeDocumentIds` (`get_agent`).
    pub async fn list(&self, input: &types::RaizKnowledgeListInput) -> Result<types::RaizKnowledgeListOutput, Error> {
        self.0.call(&operations::RAIZ_KNOWLEDGE_LIST, input).await
    }

    /// Search Knowledge
    ///
    /// Find the passages that best answer `q` in the organization's searchable documents, best first. `agentId` searches exactly what that Agent can find; `knowledgeDocumentIds` limits the search to those documents. Paused, deleted and not-yet-indexed documents are never searched. Read a whole document with `get_knowledge_document` and `expand=content`.
    pub async fn search(&self, input: &types::RaizKnowledgeSearchInput) -> Result<types::RaizKnowledgeSearchOutput, Error> {
        self.0.call(&operations::RAIZ_KNOWLEDGE_SEARCH, input).await
    }

    /// Update a Knowledge document
    ///
    /// Change a document. Omitted fields stay and `null` clears one. New `text` (text documents) or `file` (file documents) replaces the content and indexes it again; a new `sourceUrl` or `crawl` (website documents) takes a new snapshot. `status: paused` hides it from every Agent without deleting it.
    pub async fn update(&self, input: &types::RaizKnowledgeUpdateInput) -> Result<types::RaizKnowledgeUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_KNOWLEDGE_UPDATE, input).await
    }
}

/// `raiz.organization`
#[derive(Debug, Clone, Copy)]
pub struct RaizOrganizationApi<'a>(&'a Client);

impl<'a> RaizOrganizationApi<'a> {
    /// Get the organization
    ///
    /// Read your organization: name, default language and time zone, deployment capabilities, and the settings in effect (defaults included), such as how many outbound calls may run at once, who is emailed when the AI escalates a conversation or fails in it, and the daily report. capabilities.managedVoiceOutbound reports whether the shared outbound line is configured; settings.channels.managedVoiceOutboundEnabled is the organization's separate permission to use it.
    pub async fn get(&self, input: &types::RaizOrganizationGetInput) -> Result<types::RaizOrganizationGetOutput, Error> {
        self.0.call(&operations::RAIZ_ORGANIZATION_GET, input).await
    }

    /// Send the daily report now
    ///
    /// Email the daily report now to the recipients in `settings.dailyReport`, whether or not it goes out by itself each day: the calls and conversations registered in the last 24 hours, the conversations waiting for the team and those that need a review. It arrives within a minute. Fails with DAILY_REPORT_RECIPIENTS_MISSING when the report has no recipients, and DAILY_REPORT_IN_PROGRESS while a report asked for earlier is still on its way.
    pub async fn send_daily_report(&self, input: &types::RaizOrganizationSendDailyReportInput) -> Result<types::RaizOrganizationSendDailyReportOutput, Error> {
        self.0.call(&operations::RAIZ_ORGANIZATION_SEND_DAILY_REPORT, input).await
    }

    /// Update the organization
    ///
    /// Change the organization's name, default language, time zone or settings. Settings merge: an omitted setting stays and `null` returns it to its default; `notifications.recipients` is replaced as a whole. Fails with TIME_ZONE_UNKNOWN when the time zone is not a valid IANA zone (for example `America/Bogota`).
    pub async fn update(&self, input: &types::RaizOrganizationUpdateInput) -> Result<types::RaizOrganizationUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_ORGANIZATION_UPDATE, input).await
    }
}

/// `raiz.profiles`
#[derive(Debug, Clone, Copy)]
pub struct RaizProfilesApi<'a>(&'a Client);

impl<'a> RaizProfilesApi<'a> {
    /// Create a contact profile
    ///
    /// Save a named way to contact people that other systems can use by id. A `voice` profile places calls with a voice Agent, optionally with its own first message and instructions; a `whatsapp` profile opens conversations with an approved template (its parameters can use the request's variables) and its text Agent answers. Both can limit when contact starts (`contactHours`, `timeZone`). Fails with CHANNEL_AGENT_MISMATCH when the Agent's modality does not fit the channel type, and CHANNEL_TYPE_MISMATCH when the line is of another type; the template is checked when a conversation starts (find templates with `list_templates`).
    pub async fn create(&self, input: &types::RaizProfilesCreateInput) -> Result<types::RaizProfilesCreateOutput, Error> {
        self.0.call(&operations::RAIZ_PROFILES_CREATE, input).await
    }

    /// Get a contact profile
    ///
    /// Read one contact profile: its Agent and line, texts, WhatsApp template, contact hours and time zone.
    pub async fn get(&self, input: &types::RaizProfilesGetInput) -> Result<types::RaizProfilesGetOutput, Error> {
        self.0.call(&operations::RAIZ_PROFILES_GET, input).await
    }

    /// List contact profiles
    ///
    /// List the organization's contact profiles (Settings → Contact profiles), newest first: saved ways to reach someone, either a voice Agent that calls or a WhatsApp template whose text Agent answers, that other systems start conversations from by id. Filter by `agentId` or `channelType`. Conversations started from one: `list_conversations` with `profileId`.
    pub async fn list(&self, input: &types::RaizProfilesListInput) -> Result<types::RaizProfilesListOutput, Error> {
        self.0.call(&operations::RAIZ_PROFILES_LIST, input).await
    }

    /// Update a contact profile
    ///
    /// Change a contact profile's Agent, line, template, texts, contact hours, name or metadata. Omitted fields stay and `null` clears one. The channel type never changes: a field of the other type fails with CHANNEL_TYPE_MISMATCH. Conversations already started keep what they started with.
    pub async fn update(&self, input: &types::RaizProfilesUpdateInput) -> Result<types::RaizProfilesUpdateOutput, Error> {
        self.0.call(&operations::RAIZ_PROFILES_UPDATE, input).await
    }
}

/// `raiz.reports`
#[derive(Debug, Clone, Copy)]
pub struct RaizReportsApi<'a>(&'a Client);

impl<'a> RaizReportsApi<'a> {
    /// Get a report
    ///
    /// Get the numbers behind a question. `name`: `conversations_daily` (conversations the AI took part in, per day), `booking_outcomes` (verified saved booking receipts by completion day and integration, excluding tests, imports and deleted conversations; createdCount counts new appointments created by the AI, not confirmations of existing appointments; confirmedCount is its compatibility alias and must not be added again; reschedules, cancellations, failures, pending and unverified results separately), `tool_calls_daily` (Agent tool calls per day), `tool_calls` (each call, newest first), `assessment_results_daily` (assessment results per day), `pqrsf_daily` (PQRSF classification, confirmed informational resolution and correct referrals from current pqrsf_* facts; unknown and pending analysis remain visible; excludes tests/imports/deleted records), each over days in the organization's time zone (`from`, `to`; the last 30 days by default), or `campaign_progress`. Narrow with `agentId`, `channelType`, `campaignId` or `definitionId`; unsupported filters fail. At most 1000 rows: `hasMore` means split the range or filter. Booking outcomes return no partial totals when their evidence scan exceeds its bound. Conversation success and completed tool calls never establish a booking; these are historical receipts, not the provider's current appointment status.
    pub async fn get(&self, input: &types::RaizReportsGetInput) -> Result<types::RaizReportsGetOutput, Error> {
        self.0.call(&operations::RAIZ_REPORTS_GET, input).await
    }
}

/// `raiz.suppressions`
#[derive(Debug, Clone, Copy)]
pub struct RaizSuppressionsApi<'a>(&'a Client);

impl<'a> RaizSuppressionsApi<'a> {
    /// Block contact
    ///
    /// Forbid contacting a phone number, every number with a prefix, an IP network (web chat) or a country. Calls, WhatsApp messages and the web widget check these rules before contacting anyone. Give a `reason` people will understand later. Fails with SUPPRESSION_EXISTS when the same rule already exists.
    pub async fn create(&self, input: &types::RaizSuppressionsCreateInput) -> Result<types::RaizSuppressionsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_SUPPRESSIONS_CREATE, input).await
    }

    /// List blocked contacts
    ///
    /// List the organization's do-not-contact rules (Settings → Block list), newest first: blocked phone numbers, number prefixes, IP networks and countries. Filter by `kind`, `phone` or `q`. Use it to explain why a call or message was not sent (CONTACT_SUPPRESSED).
    pub async fn list(&self, input: &types::RaizSuppressionsListInput) -> Result<types::RaizSuppressionsListOutput, Error> {
        self.0.call(&operations::RAIZ_SUPPRESSIONS_LIST, input).await
    }
}

/// `raiz.tasks`
#[derive(Debug, Clone, Copy)]
pub struct RaizTasksApi<'a>(&'a Client);

impl<'a> RaizTasksApi<'a> {
    /// Cancel a task
    ///
    /// Cancel a background task. A queued task is cancelled at once. A running one is asked to stop: it stays `running` with `cancelRequestedAt` until its next checkpoint (an import stops after its current batch of rows), then ends `cancelled`; what it already did stays done. Asking again changes nothing; a finished task fails with INVALID_STATE.
    pub async fn cancel(&self, input: &types::RaizTasksCancelInput) -> Result<types::RaizTasksCancelOutput, Error> {
        self.0.call(&operations::RAIZ_TASKS_CANCEL, input).await
    }

    /// Get a task
    ///
    /// Read one background task: status, step progress and, once it ends, its result or `errorCode`. Check it again every few seconds after an action returns a task, such as `import_contacts`.
    pub async fn get(&self, input: &types::RaizTasksGetInput) -> Result<types::RaizTasksGetOutput, Error> {
        self.0.call(&operations::RAIZ_TASKS_GET, input).await
    }

    /// List background tasks
    ///
    /// List the organization's background tasks, newest first: contact imports (`contact_import`), conversation re-analyses (`analysis_run`) and website crawls (`knowledge_crawl`). Filter by `kind` or `status` (`queued`, `running`, `completed`, `failed`, `cancelled`). Next: `get_task` for one task's progress or error.
    pub async fn list(&self, input: &types::RaizTasksListInput) -> Result<types::RaizTasksListOutput, Error> {
        self.0.call(&operations::RAIZ_TASKS_LIST, input).await
    }

    /// Retry a task
    ///
    /// Retry a failed task whose kind allows it (`retryable: true`): contact imports, re-analyses and website crawls. It is queued again and resumes from where it stopped (an import after its last imported row), keeping its attempt count; the history records who retried it. Fails with INVALID_STATE when the task did not fail or newer work for the same resource is pending, and TASK_NOT_RETRYABLE when its kind never retries or it used every attempt.
    pub async fn retry(&self, input: &types::RaizTasksRetryInput) -> Result<types::RaizTasksRetryOutput, Error> {
        self.0.call(&operations::RAIZ_TASKS_RETRY, input).await
    }
}

/// `raiz.templates`
#[derive(Debug, Clone, Copy)]
pub struct RaizTemplatesApi<'a>(&'a Client);

impl<'a> RaizTemplatesApi<'a> {
    /// Create a WhatsApp template
    ///
    /// Submit a new message template for WhatsApp's review in a channel's business account: `name`, `language`, `category` (`marketing` or `utility`), `body`, and optional `header`, `footer` and `buttons`. The `header` is a fixed text, or a document (PDF), image or video with a sample file in base64 (up to 5 MiB) for the review; each send then carries its own file. It stays `pending` until WhatsApp approves or rejects it, usually within minutes, and WhatsApp may change its category. Check the result with `list_templates`; a rejected template explains why in `rejectedReason`.
    pub async fn create(&self, input: &types::RaizTemplatesCreateInput) -> Result<types::RaizTemplatesCreateOutput, Error> {
        self.0.call(&operations::RAIZ_TEMPLATES_CREATE, input).await
    }

    /// List WhatsApp templates
    ///
    /// List a WhatsApp channel's message templates, read live from WhatsApp, newest first (`channelId` from `list_channels`; filter by `status`). `sendable` says whether Raíz can send one (approved, with variables only in the body), `parameterCount` how many body values it needs, and a `document`, `image` or `video` `header` that each send also carries its file. A template is required to start a WhatsApp conversation with someone who has not written in the last 24 hours.
    pub async fn list(&self, input: &types::RaizTemplatesListInput) -> Result<types::RaizTemplatesListOutput, Error> {
        self.0.call(&operations::RAIZ_TEMPLATES_LIST, input).await
    }
}

/// `raiz.voices`
#[derive(Debug, Clone, Copy)]
pub struct RaizVoicesApi<'a>(&'a Client);

impl<'a> RaizVoicesApi<'a> {
    /// List voices
    ///
    /// Find a voice for a voice Agent. Each voice shows the languages it is verified for and a sample to listen to (`previewUrl`), sorted by name. Search with `q` (name or label) and keep the voices of one language with `language`. Next: set the chosen id as the Agent's `settings.voice.voiceId` with `update_agent`.
    pub async fn list(&self, input: &types::RaizVoicesListInput) -> Result<types::RaizVoicesListOutput, Error> {
        self.0.call(&operations::RAIZ_VOICES_LIST, input).await
    }
}

/// `raiz`
#[derive(Debug, Clone, Copy)]
pub struct RaizApi<'a>(&'a Client);

impl<'a> RaizApi<'a> {
    pub fn agents(&self) -> RaizAgentsApi<'a> {
        RaizAgentsApi(self.0)
    }

    pub fn bulletins(&self) -> RaizBulletinsApi<'a> {
        RaizBulletinsApi(self.0)
    }

    pub fn campaigns(&self) -> RaizCampaignsApi<'a> {
        RaizCampaignsApi(self.0)
    }

    pub fn changes(&self) -> RaizChangesApi<'a> {
        RaizChangesApi(self.0)
    }

    pub fn channels(&self) -> RaizChannelsApi<'a> {
        RaizChannelsApi(self.0)
    }

    pub fn contacts(&self) -> RaizContactsApi<'a> {
        RaizContactsApi(self.0)
    }

    pub fn conversations(&self) -> RaizConversationsApi<'a> {
        RaizConversationsApi(self.0)
    }

    pub fn definitions(&self) -> RaizDefinitionsApi<'a> {
        RaizDefinitionsApi(self.0)
    }

    pub fn eval_cases(&self) -> RaizEvalCasesApi<'a> {
        RaizEvalCasesApi(self.0)
    }

    pub fn eval_runs(&self) -> RaizEvalRunsApi<'a> {
        RaizEvalRunsApi(self.0)
    }

    pub fn facts(&self) -> RaizFactsApi<'a> {
        RaizFactsApi(self.0)
    }

    pub fn knowledge(&self) -> RaizKnowledgeApi<'a> {
        RaizKnowledgeApi(self.0)
    }

    pub fn organization(&self) -> RaizOrganizationApi<'a> {
        RaizOrganizationApi(self.0)
    }

    pub fn profiles(&self) -> RaizProfilesApi<'a> {
        RaizProfilesApi(self.0)
    }

    pub fn reports(&self) -> RaizReportsApi<'a> {
        RaizReportsApi(self.0)
    }

    pub fn suppressions(&self) -> RaizSuppressionsApi<'a> {
        RaizSuppressionsApi(self.0)
    }

    pub fn tasks(&self) -> RaizTasksApi<'a> {
        RaizTasksApi(self.0)
    }

    pub fn templates(&self) -> RaizTemplatesApi<'a> {
        RaizTemplatesApi(self.0)
    }

    pub fn voices(&self) -> RaizVoicesApi<'a> {
        RaizVoicesApi(self.0)
    }
}

/// `salud-total.affiliations`
#[derive(Debug, Clone, Copy)]
pub struct SaludTotalAffiliationsApi<'a>(&'a Client);

impl<'a> SaludTotalAffiliationsApi<'a> {
    /// Look up one member’s IPS affiliation
    ///
    /// Consult current affiliation for one exact CC document using an active tenant connection. Returns only the provider status for a unique match. No match and ambiguity are distinct from outages. This read does not authorize treatment.
    pub async fn lookup(&self, input: &types::SaludTotalAffiliationsLookupInput) -> Result<types::SaludTotalAffiliationsLookupOutput, Error> {
        self.0.call(&operations::SALUD_TOTAL_AFFILIATIONS_LOOKUP, input).await
    }
}

/// `salud-total.authorizations`
#[derive(Debug, Clone, Copy)]
pub struct SaludTotalAuthorizationsApi<'a>(&'a Client);

impl<'a> SaludTotalAuthorizationsApi<'a> {
    /// Look up authorization records for one member, site and day
    ///
    /// Availability of authorization records for the matched member and selected site on one calendar day. A missing row is not a denial. Returns a count or location_required with permitted choices. Start without branchId or siteId. For location_required, repeat with the chosen ID in field, or pass nextChoiceCursor as choiceCursor for more choices.
    pub async fn lookup(&self, input: &types::SaludTotalAuthorizationsLookupInput) -> Result<types::SaludTotalAuthorizationsLookupOutput, Error> {
        self.0.call(&operations::SALUD_TOTAL_AUTHORIZATIONS_LOOKUP, input).await
    }
}

/// `salud-total.filings`
#[derive(Debug, Clone, Copy)]
pub struct SaludTotalFilingsApi<'a>(&'a Client);

impl<'a> SaludTotalFilingsApi<'a> {
    /// Look up filing records for one member and day
    ///
    /// Availability of requests for the matched member and IPS on one calendar day. The provider filing search is institution-wide after site admission. Returns a count or location_required with permitted choices. Start without branchId or siteId. For location_required, repeat with the chosen ID in field, or pass nextChoiceCursor as choiceCursor for more choices.
    pub async fn lookup(&self, input: &types::SaludTotalFilingsLookupInput) -> Result<types::SaludTotalFilingsLookupOutput, Error> {
        self.0.call(&operations::SALUD_TOTAL_FILINGS_LOOKUP, input).await
    }
}

/// `salud-total.invoices`
#[derive(Debug, Clone, Copy)]
pub struct SaludTotalInvoicesApi<'a>(&'a Client);

impl<'a> SaludTotalInvoicesApi<'a> {
    /// Look up an exact IPS invoice reference
    ///
    /// Look up one exact invoice prefix and number in the active billing connection’s institution. Returns the number of matching rows. Outside the provider’s live reporting hours, REPORTING_WINDOW_CLOSED gives the opening hour (inclusive), closing hour (exclusive) and time zone. Payment or rejection requires separate evidence.
    pub async fn lookup(&self, input: &types::SaludTotalInvoicesLookupInput) -> Result<types::SaludTotalInvoicesLookupOutput, Error> {
        self.0.call(&operations::SALUD_TOTAL_INVOICES_LOOKUP, input).await
    }
}

/// `salud-total.notices`
#[derive(Debug, Clone, Copy)]
pub struct SaludTotalNoticesApi<'a>(&'a Client);

impl<'a> SaludTotalNoticesApi<'a> {
    /// Count IPS objection and devolution notices
    ///
    /// Read the IPS objection/devolution notification total from the billing connection. Returns only a count. Never opens or acknowledges a notification.
    pub async fn count(&self, input: &types::SaludTotalNoticesCountInput) -> Result<types::SaludTotalNoticesCountOutput, Error> {
        self.0.call(&operations::SALUD_TOTAL_NOTICES_COUNT, input).await
    }
}

/// `salud-total`
#[derive(Debug, Clone, Copy)]
pub struct SaludTotalApi<'a>(&'a Client);

impl<'a> SaludTotalApi<'a> {
    pub fn affiliations(&self) -> SaludTotalAffiliationsApi<'a> {
        SaludTotalAffiliationsApi(self.0)
    }

    pub fn authorizations(&self) -> SaludTotalAuthorizationsApi<'a> {
        SaludTotalAuthorizationsApi(self.0)
    }

    pub fn filings(&self) -> SaludTotalFilingsApi<'a> {
        SaludTotalFilingsApi(self.0)
    }

    pub fn invoices(&self) -> SaludTotalInvoicesApi<'a> {
        SaludTotalInvoicesApi(self.0)
    }

    pub fn notices(&self) -> SaludTotalNoticesApi<'a> {
        SaludTotalNoticesApi(self.0)
    }
}

/// `somos.appointments`
#[derive(Debug, Clone, Copy)]
pub struct SomosAppointmentsApi<'a>(&'a Client);

impl<'a> SomosAppointmentsApi<'a> {
    /// Cancel an appointment
    ///
    /// Cancel one exact future appointment, only after the person explicitly confirmed it, with the cancellation reference from a fresh patient lookup. The clinic requires two hours of notice. Never repeat a cancellation whose outcome is unknown. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn cancel(&self, input: &types::SomosAppointmentsCancelInput) -> Result<types::SomosAppointmentsCancelOutput, Error> {
        self.0.call(&operations::SOMOS_APPOINTMENTS_CANCEL, input).await
    }

    /// Book an appointment
    ///
    /// Book one slot somos.availability.lookup returned, only after the person explicitly confirmed that exact appointment. Pass its reference unchanged. A service the clinic books with a companion is booked with it, in consecutive slots, in this one call. Never repeat a booking whose outcome is unknown. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn create(&self, input: &types::SomosAppointmentsCreateInput) -> Result<types::SomosAppointmentsCreateOutput, Error> {
        self.0.call(&operations::SOMOS_APPOINTMENTS_CREATE, input).await
    }
}

/// `somos.availability`
#[derive(Debug, Clone, Copy)]
pub struct SomosAvailabilityApi<'a>(&'a Client);

impl<'a> SomosAvailabilityApi<'a> {
    /// Find available appointments
    ///
    /// Real available appointments for a service under a contract, one slot per item in chronological order, each with its professional and the booking reference (token) somos.appointments.create takes. A service that must be booked with a companion (logoaudiometry after its audiometry) offers only the slots whose companion is free too. limit is how many slots to offer (three in a conversation); for more, call again with the same arguments and pageInfo.nextCursor. window: requested; extended, the dates asked for had none; alternative, the time preference had none (say those first). A completed search without slots is TOOL_REFUSED (NO_AVAILABILITY); never search again without the person's filters. This read never books anything. A refusal is TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead; it is never an outage. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn lookup(&self, input: &types::SomosAvailabilityLookupInput) -> Result<types::SomosAvailabilityLookupOutput, Error> {
        self.0.call(&operations::SOMOS_AVAILABILITY_LOOKUP, input).await
    }
}

/// `somos.clinic`
#[derive(Debug, Clone, Copy)]
pub struct SomosClinicApi<'a>(&'a Client);

impl<'a> SomosClinicApi<'a> {
    /// Get the clinic
    ///
    /// The clinic of the connection a call names, or of the organization's only one: its name, its SIIPS address, its timezone (every date and time the clinical operations answer is local to it) and the version of its tool definitions (somos.tools.list). An Agent reads it first in a conversation, to know the clinic and its time. A connection of another clinic is CLINIC_NOT_SERVED; another organization's is CONNECTION_NOT_FOUND. Contains no patient data and performs no clinical login.
    pub async fn get(&self, input: &types::SomosClinicGetInput) -> Result<types::SomosClinicGetOutput, Error> {
        self.0.call(&operations::SOMOS_CLINIC_GET, input).await
    }
}

/// `somos.contracts`
#[derive(Debug, Clone, Copy)]
pub struct SomosContractsApi<'a>(&'a Client);

impl<'a> SomosContractsApi<'a> {
    /// List the clinic's contracts
    ///
    /// The clinic's current contracts and agreements, in ascending code order. A contract with covers schedules only those specialties: another one is in a sibling contract of the same payer. A payer name that matches no contract, or several, is TOOL_REFUSED (CONTRACT_NOT_FOUND, CONTRACT_AMBIGUOUS) with the closest names in details.didYouMean, to offer the person. Preserve the returned identifiers; a failed lookup never proves a patient lacks coverage. A refusal is TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead; it is never an outage. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn list(&self, input: &types::SomosContractsListInput) -> Result<types::SomosContractsListOutput, Error> {
        self.0.call(&operations::SOMOS_CONTRACTS_LIST, input).await
    }
}

/// `somos.groups`
#[derive(Debug, Clone, Copy)]
pub struct SomosGroupsApi<'a>(&'a Client);

impl<'a> SomosGroupsApi<'a> {
    /// List therapy groups
    ///
    /// The current therapy groups of a specialty, each with how it matched the name the person said: exact is their group; suggested and all are options the person recognizes theirs among, never one an Agent picks. The age validates a group and never chooses it. Never invent a group or an age range. A refusal is TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead; it is never an outage. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn list(&self, input: &types::SomosGroupsListInput) -> Result<types::SomosGroupsListOutput, Error> {
        self.0.call(&operations::SOMOS_GROUPS_LIST, input).await
    }
}

/// `somos.patients`
#[derive(Debug, Clone, Copy)]
pub struct SomosPatientsApi<'a>(&'a Client);

impl<'a> SomosPatientsApi<'a> {
    /// Correct a patient's details
    ///
    /// Update only the patient fields the person explicitly corrected and confirmed; omit unchanged fields. Never repeat an update whose outcome is unknown. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn correct(&self, input: &types::SomosPatientsCorrectInput) -> Result<types::SomosPatientsCorrectOutput, Error> {
        self.0.call(&operations::SOMOS_PATIENTS_CORRECT, input).await
    }

    /// Register a patient
    ///
    /// Register a patient only after a lookup confirmed it is not found and the person explicitly confirmed the required details. Never repeat a registration whose outcome is unknown. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn create(&self, input: &types::SomosPatientsCreateInput) -> Result<types::SomosPatientsCreateOutput, Error> {
        self.0.call(&operations::SOMOS_PATIENTS_CREATE, input).await
    }

    /// Look up a patient
    ///
    /// The patient's profile, recent and upcoming appointments (each upcoming one with its cancellation reference), by exact document number, after the clinic's identity and consent checks. found:false is an expected answer: confirm the document and offer to register the patient. Reuse what the patient already confirmed and avoid unrelated patient data. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn lookup(&self, input: &types::SomosPatientsLookupInput) -> Result<types::SomosPatientsLookupOutput, Error> {
        self.0.call(&operations::SOMOS_PATIENTS_LOOKUP, input).await
    }
}

/// `somos.services`
#[derive(Debug, Clone, Copy)]
pub struct SomosServicesApi<'a>(&'a Client);

impl<'a> SomosServicesApi<'a> {
    /// List a contract's services
    ///
    /// The services of one of the clinic's contracts (code), in ascending code order; with q, only those whose code or name contains it. When none does, the search is TOOL_REFUSED (SERVICE_SEARCH_NOT_FOUND), and data.details.didYouMean holds up to three of the contract's services, within the same filters, that Cloudflare AI Search finds closest in meaning: suggestions to confirm, never substitutes. A contract without the service asked for is TOOL_REFUSED (SERVICE_NOT_IN_CONTRACT), whose guidance names the contract that has it. Catalog items that cannot be booked are not appointment availability. A refusal is TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead; it is never an outage. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn list(&self, input: &types::SomosServicesListInput) -> Result<types::SomosServicesListOutput, Error> {
        self.0.call(&operations::SOMOS_SERVICES_LIST, input).await
    }

    /// Look up a contract's services
    ///
    /// The services of one of the clinic's contracts closest to a query: first those whose code or name contains it (match words, score 1), then those Cloudflare AI Search finds close in meaning (match meaning), at most limit (5 by default). Every answer is one of the contract's current services, within specialty when it is named; a match by meaning is a suggestion to confirm with the person or the order, never a substitute for the service ordered. An empty answer says nothing about coverage. CONTRACT_NOT_FOUND when the clinic has no such contract; SERVICES_SEARCH_UNAVAILABLE when AI Search cannot answer and no service matched by words. Contains no patient data.
    pub async fn lookup(&self, input: &types::SomosServicesLookupInput) -> Result<types::SomosServicesLookupOutput, Error> {
        self.0.call(&operations::SOMOS_SERVICES_LOOKUP, input).await
    }
}

/// `somos.specialtyRules`
#[derive(Debug, Clone, Copy)]
pub struct SomosSpecialtyRulesApi<'a>(&'a Client);

impl<'a> SomosSpecialtyRulesApi<'a> {
    /// Get a specialty's rules
    ///
    /// A specialty's prerequisites and scheduling rules, as the clinic publishes them. Somos Salud's tool pack sets this tool's rules, parameters and accepted values (somos.tools.list answers them), and every call is checked against them.
    pub async fn get(&self, input: &types::SomosSpecialtyRulesGetInput) -> Result<types::SomosSpecialtyRulesGetOutput, Error> {
        self.0.call(&operations::SOMOS_SPECIALTY_RULES_GET, input).await
    }
}

/// `somos.tools`
#[derive(Debug, Clone, Copy)]
pub struct SomosToolsApi<'a>(&'a Client);

impl<'a> SomosToolsApi<'a> {
    /// List the clinic's tools
    ///
    /// Every tool Somos Salud's agents may call, in ascending name order, with the clinic's exact parameters and instructions, each naming the operation that runs it; a list's tool declares the page fields too. The clinic itself, its timezone and the version of these definitions are somos.clinic.get's. Contains no patient data and performs no clinical login.
    pub async fn list(&self, input: &types::SomosToolsListInput) -> Result<types::SomosToolsListOutput, Error> {
        self.0.call(&operations::SOMOS_TOOLS_LIST, input).await
    }
}

/// `somos`
#[derive(Debug, Clone, Copy)]
pub struct SomosApi<'a>(&'a Client);

impl<'a> SomosApi<'a> {
    pub fn appointments(&self) -> SomosAppointmentsApi<'a> {
        SomosAppointmentsApi(self.0)
    }

    pub fn availability(&self) -> SomosAvailabilityApi<'a> {
        SomosAvailabilityApi(self.0)
    }

    pub fn clinic(&self) -> SomosClinicApi<'a> {
        SomosClinicApi(self.0)
    }

    pub fn contracts(&self) -> SomosContractsApi<'a> {
        SomosContractsApi(self.0)
    }

    pub fn groups(&self) -> SomosGroupsApi<'a> {
        SomosGroupsApi(self.0)
    }

    pub fn patients(&self) -> SomosPatientsApi<'a> {
        SomosPatientsApi(self.0)
    }

    pub fn services(&self) -> SomosServicesApi<'a> {
        SomosServicesApi(self.0)
    }

    pub fn specialty_rules(&self) -> SomosSpecialtyRulesApi<'a> {
        SomosSpecialtyRulesApi(self.0)
    }

    pub fn tools(&self) -> SomosToolsApi<'a> {
        SomosToolsApi(self.0)
    }
}

/// `sura.affiliations`
#[derive(Debug, Clone, Copy)]
pub struct SuraAffiliationsApi<'a>(&'a Client);

impl<'a> SuraAffiliationsApi<'a> {
    /// Look up eligibility
    ///
    /// Look up affiliate rights by exact document. Eligible coverage does not authorize a procedure or determine patient charges. A noContract outcome is distinct from inactive or unavailable.
    pub async fn lookup(&self, input: &types::SuraAffiliationsLookupInput) -> Result<types::SuraAffiliationsLookupOutput, Error> {
        self.0.call(&operations::SURA_AFFILIATIONS_LOOKUP, input).await
    }
}

/// `sura.appointments`
#[derive(Debug, Clone, Copy)]
pub struct SuraAppointmentsApi<'a>(&'a Client);

impl<'a> SuraAppointmentsApi<'a> {
    /// Cancel an appointment
    ///
    /// Cancel one appointment the patient already has, identified by its moment: startsAt, exactly as list_appointments answered it (or the former date and time). Confirm with the caller which appointment, in their own words, before calling. Only appointments at this clinic can be cancelled here. ok:true with cancelled:true means it is gone; anything else means it is not, so never tell the caller it was cancelled without that.
    pub async fn cancel(&self, input: &types::SuraAppointmentsCancelInput) -> Result<types::SuraAppointmentsCancelOutput, Error> {
        self.0.call(&operations::SURA_APPOINTMENTS_CANCEL, input).await
    }

    /// Book an appointment
    ///
    /// Book one appointment at an exact moment for one patient: startsAt, exactly as an option or the ready booking arguments answered it (or the former date and time). Only call it after the caller has confirmed the day, the hour and the service out loud. The moment must be one that find_availability just returned; the tool re-checks it against the live agenda and refuses if it was taken in the meantime. ok:true with booked:true means the appointment exists.
    pub async fn create(&self, input: &types::SuraAppointmentsCreateInput) -> Result<types::SuraAppointmentsCreateOutput, Error> {
        self.0.call(&operations::SURA_APPOINTMENTS_CREATE, input).await
    }

    /// List appointments
    ///
    /// List the appointments this patient still has coming up, at every clinic, as a page in date and hour order: up to 25 by default, and while pageInfo.hasNextPage is true call again with cursor set to pageInfo.nextCursor to read the rest before telling the caller what they have. Use it before cancelling, and whenever the caller asks what they have. Read out only the date, the hour and the service; to cancel one, pass its startsAt to cancel_appointment. Entries with cancellable:false belong to a different clinic and cannot be cancelled from this line — read them out, but send the caller to that clinic to change them. An empty page means the patient has nothing pending (offer to book: ask which service and use find_availability); a document the portal does not know answers found:false instead, never an empty page.
    pub async fn lookup(&self, input: &types::SuraAppointmentsLookupInput) -> Result<types::SuraAppointmentsLookupOutput, Error> {
        self.0.call(&operations::SURA_APPOINTMENTS_LOOKUP, input).await
    }
}

/// `sura.authorizations`
#[derive(Debug, Clone, Copy)]
pub struct SuraAuthorizationsApi<'a>(&'a Client);

impl<'a> SuraAuthorizationsApi<'a> {
    /// Look up an authorization
    ///
    /// Read an existing SaludWeb authorization for the exact CC patient, issuer and authorization number. Reports delivered status, dates, explicit charge type, fixed fee or copay percentage and provider-reported payments. Unknown balance and copay cap remain null. Does not issue, attend, modify or pay an authorization.
    pub async fn lookup(&self, input: &types::SuraAuthorizationsLookupInput) -> Result<types::SuraAuthorizationsLookupOutput, Error> {
        self.0.call(&operations::SURA_AUTHORIZATIONS_LOOKUP, input).await
    }
}

/// `sura.availability`
#[derive(Debug, Clone, Copy)]
pub struct SuraAvailabilityApi<'a>(&'a Client);

impl<'a> SuraAvailabilityApi<'a> {
    /// Find availability
    ///
    /// Find the free appointment times for one patient and one service. With no date it answers with the soonest day that has room; with a date it answers for that day. times[] are the hours you may book, otherDates[] are other days that month with room, and otherProfessionals[] are the other doctors who see this service. Offer two or three times, never the whole list.
    pub async fn lookup(&self, input: &types::SuraAvailabilityLookupInput) -> Result<types::SuraAvailabilityLookupOutput, Error> {
        self.0.call(&operations::SURA_AVAILABILITY_LOOKUP, input).await
    }
}

/// `sura.bookingOptions`
#[derive(Debug, Clone, Copy)]
pub struct SuraBookingOptionsApi<'a>(&'a Client);

impl<'a> SuraBookingOptionsApi<'a> {
    /// Prepare an appointment
    ///
    /// Prepare a SURA appointment in ONE read-only call. With patient identity and the ordered service, first checks existing appointments; if one matches, returns it and stops. Otherwise resolves patient, live service/area and professional names internally and reads availability, checking bounded date and professional alternatives while preserving explicit doctor/date/expiry constraints. Returns patient.primaryIps, actual service/professional, times and ready booking arguments. Use after the required document review, instead of chaining list_appointments, lookup_patient, list_services and find_availability. It NEVER books, cancels, changes an appointment or verifies the documents for you. Apply clinic eligibility to the result, offer up to three chronological verified options from options; nextDates are calendar hints that require an hour lookup, never confirmed slots. Use fromDate for later alternatives and preserve explicit date, doctor and time preferences. Propose an actual slot and obtain one confirmation before the separate book_appointment call. Existing-appointment-only questions still use list_appointments.
    pub async fn lookup(&self, input: &types::SuraBookingOptionsLookupInput) -> Result<types::SuraBookingOptionsLookupOutput, Error> {
        self.0.call(&operations::SURA_BOOKING_OPTIONS_LOOKUP, input).await
    }
}

/// `sura.connections`
#[derive(Debug, Clone, Copy)]
pub struct SuraConnectionsApi<'a>(&'a Client);

impl<'a> SuraConnectionsApi<'a> {
    /// Remove a SURA connection
    ///
    /// Drop one of the organization's SURA connections, by its connectionId: every portal user's session, cooldown and flow lock. Account calls it before it forgets the connection; operations on it then answer CONNECTION_NOT_FOUND or CONNECTION_NOT_CONFIGURED. Its booking ledger stays.
    pub async fn delete(&self, input: &types::SuraConnectionsDeleteInput) -> Result<types::SuraConnectionsDeleteOutput, Error> {
        self.0.call(&operations::SURA_CONNECTIONS_DELETE, input).await
    }

    /// Get a SURA connection
    ///
    /// One of the organization's SURA connections, by its connectionId: its status, the portal, the institution it schedules in, each portal user's status with its masked username, and when SURA last accepted a login. Never a credential.
    pub async fn get(&self, input: &types::SuraConnectionsGetInput) -> Result<types::SuraConnectionsGetOutput, Error> {
        self.0.call(&operations::SURA_CONNECTIONS_GET, input).await
    }

    /// List the SURA connections
    ///
    /// The organization's SURA connections, as Account lists them, a page at a time in the order of their ids, each with where it stands and its portal users' status. Never a credential.
    pub async fn list(&self, input: &types::SuraConnectionsListInput) -> Result<types::SuraConnectionsListOutput, Error> {
        self.0.call(&operations::SURA_CONNECTIONS_LIST, input).await
    }

    /// Verify a SURA connection
    ///
    /// Prove a SURA connection before Account seals it: each portal user sent is asked once, with a real login, and keeps the session it opens. Answers active when at least one user is accepted (with the object the connection runs on, and each user's answer), institution_required when the Agenda users see several institutions (call again naming one, with the same users: no second login), or rejected when SURA refused every user. A refused login is never retried, and changes nothing.
    pub async fn verify(&self, input: &types::SuraConnectionsVerifyInput) -> Result<types::SuraConnectionsVerifyOutput, Error> {
        self.0.call(&operations::SURA_CONNECTIONS_VERIFY, input).await
    }
}

/// `sura.copays`
#[derive(Debug, Clone, Copy)]
pub struct SuraCopaysApi<'a>(&'a Client);

impl<'a> SuraCopaysApi<'a> {
    /// List copay history
    ///
    /// Read the exact CC patient's annual copay utilization history. Accumulated copays are historical amounts, not an outstanding balance or a current voucher quote. No history does not imply exemption. Incomplete or unknown provider responses fail closed. Never updates accumulated copays.
    pub async fn lookup(&self, input: &types::SuraCopaysLookupInput) -> Result<types::SuraCopaysLookupOutput, Error> {
        self.0.call(&operations::SURA_COPAYS_LOOKUP, input).await
    }
}

/// `sura.operations`
#[derive(Debug, Clone, Copy)]
pub struct SuraOperationsApi<'a>(&'a Client);

impl<'a> SuraOperationsApi<'a> {
    /// Get an operation
    ///
    /// Get the result of an earlier call that answered status "pending", by its operationId. SURA's portal does one thing at a time per account, so a call may wait its turn; it keeps running even if you stop waiting. Call this after retryAfterSeconds. It waits briefly and answers pending again while the operation is still running. Repeating the original call with the same arguments also returns the same operation.
    pub async fn get(&self, input: &types::SuraOperationsGetInput) -> Result<types::SuraOperationsGetOutput, Error> {
        self.0.call(&operations::SURA_OPERATIONS_GET, input).await
    }
}

/// `sura.patients`
#[derive(Debug, Clone, Copy)]
pub struct SuraPatientsApi<'a>(&'a Client);

impl<'a> SuraPatientsApi<'a> {
    /// Look up a patient
    ///
    /// Find one patient by exact document type and number. The match is exact: there is no partial or name search. found:false is an expected result, not an error — confirm the document type and the number with the caller and try again. Returns the patient's names, age, sex, city, entitlement status, and primaryIps: the primary-care IPS SURA has assigned to this person, read from the portal so the caller never has to know it. The health plan is never a question for the caller: the portal assigns the coverage itself and the appointment tools read it from the live form, so do not ask which plan or coverage to use and do not pass one anywhere.
    pub async fn lookup(&self, input: &types::SuraPatientsLookupInput) -> Result<types::SuraPatientsLookupOutput, Error> {
        self.0.call(&operations::SURA_PATIENTS_LOOKUP, input).await
    }
}

/// `sura.professionals`
#[derive(Debug, Clone, Copy)]
pub struct SuraProfessionalsApi<'a>(&'a Client);

impl<'a> SuraProfessionalsApi<'a> {
    /// List registered professionals
    ///
    /// List SURA's registered professionals for a service at this connection's institution, including doctors without published or free appointments. First obtain serviceId and optionally areaId from list_services. No patient identity is needed. Follow pageInfo.nextCursor while hasNextPage is true before concluding a name is absent from this scoped directory. Absence does not establish that a doctor does not exist or works nowhere else. Registration does not establish coverage, authorization, or availability: use prepare_appointment with the patient's evidence and the requested professional name to find actual slots. Never substitute another doctor without the patient's agreement. An unreadable or incomplete provider directory is a technical failure, not an empty list. A refusal, or a call still running, is the failure TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead (OPERATION_PENDING: poll data.details.operationId with operations.get); it is never an outage.
    pub async fn list(&self, input: &types::SuraProfessionalsListInput) -> Result<types::SuraProfessionalsListOutput, Error> {
        self.0.call(&operations::SURA_PROFESSIONALS_LIST, input).await
    }
}

/// `sura.services`
#[derive(Debug, Clone, Copy)]
pub struct SuraServicesApi<'a>(&'a Client);

impl<'a> SuraServicesApi<'a> {
    /// List services
    ///
    /// List what this clinic can be booked for. Call it with no arguments to get every service it offers, each with the areaId it belongs to, as a page ordered by areaId and then id: up to 25 by default (limit up to 100), and while pageInfo.hasNextPage is true call again with cursor set to pageInfo.nextCursor before deciding a service is not offered. Match the caller's own words to a service here rather than inventing one, and carry both id and areaId into find_availability; never substitute an exam with a consultation. A service missing from the catalog does not mean the clinic does not provide it, or that it does not exist at other sites or channels. The catalog is not filtered by plan or coverage: the portal applies the patient's own coverage when availability is read. A refusal, or a call still running, is the failure TOOL_REFUSED: data.reason says why, data.guidance what to do next and data.details what it offers instead (OPERATION_PENDING: poll data.details.operationId with operations.get); it is never an outage.
    pub async fn list(&self, input: &types::SuraServicesListInput) -> Result<types::SuraServicesListOutput, Error> {
        self.0.call(&operations::SURA_SERVICES_LIST, input).await
    }
}

/// `sura`
#[derive(Debug, Clone, Copy)]
pub struct SuraApi<'a>(&'a Client);

impl<'a> SuraApi<'a> {
    pub fn affiliations(&self) -> SuraAffiliationsApi<'a> {
        SuraAffiliationsApi(self.0)
    }

    pub fn appointments(&self) -> SuraAppointmentsApi<'a> {
        SuraAppointmentsApi(self.0)
    }

    pub fn authorizations(&self) -> SuraAuthorizationsApi<'a> {
        SuraAuthorizationsApi(self.0)
    }

    pub fn availability(&self) -> SuraAvailabilityApi<'a> {
        SuraAvailabilityApi(self.0)
    }

    pub fn booking_options(&self) -> SuraBookingOptionsApi<'a> {
        SuraBookingOptionsApi(self.0)
    }

    pub fn connections(&self) -> SuraConnectionsApi<'a> {
        SuraConnectionsApi(self.0)
    }

    pub fn copays(&self) -> SuraCopaysApi<'a> {
        SuraCopaysApi(self.0)
    }

    pub fn operations(&self) -> SuraOperationsApi<'a> {
        SuraOperationsApi(self.0)
    }

    pub fn patients(&self) -> SuraPatientsApi<'a> {
        SuraPatientsApi(self.0)
    }

    pub fn professionals(&self) -> SuraProfessionalsApi<'a> {
        SuraProfessionalsApi(self.0)
    }

    pub fn services(&self) -> SuraServicesApi<'a> {
        SuraServicesApi(self.0)
    }
}

/// `visoft.appointments`
#[derive(Debug, Clone, Copy)]
pub struct VisoftAppointmentsApi<'a>(&'a Client);

impl<'a> VisoftAppointmentsApi<'a> {
    /// Cancel an appointment
    ///
    /// Cancel one of the patient's appointments, by the reference a fresh visoft.patients.lookup answered in history.appointments. The patient's agenda is read again right before the write, and an appointment no longer in it is APPOINTMENT_NOT_CANCELLABLE. minimumNoticeMinutes refuses one that starts sooner (CANCEL_NOTICE_REQUIRED). CANCELLATION_REFUSED: SIIPS did not confirm it (data.message says why). A cancellation whose outcome is unknown answers OUTCOME_UNKNOWN: read the patient's agenda before anything else, never repeat it blindly.
    pub async fn cancel(&self, input: &types::VisoftAppointmentsCancelInput) -> Result<types::VisoftAppointmentsCancelOutput, Error> {
        self.0.call(&operations::VISOFT_APPOINTMENTS_CANCEL, input).await
    }

    /// Book an appointment
    ///
    /// Book one free slot visoft.availability.lookup answered, by its reference and the contract and service of that search, for the patient with that document. APPOINTMENT_REQUEST_EXISTS: the patient already has a requested appointment for the service. BOOKING_REFUSED: SIIPS refused it (data.message says why). Nothing is booked in either case. A booking whose outcome is unknown answers OUTCOME_UNKNOWN: read the patient's agenda before anything else, never book it again blindly.
    pub async fn create(&self, input: &types::VisoftAppointmentsCreateInput) -> Result<types::VisoftAppointmentsCreateOutput, Error> {
        self.0.call(&operations::VISOFT_APPOINTMENTS_CREATE, input).await
    }
}

/// `visoft.availability`
#[derive(Debug, Clone, Copy)]
pub struct VisoftAvailabilityApi<'a>(&'a Client);

impl<'a> VisoftAvailabilityApi<'a> {
    /// Find free slots
    ///
    /// The free slots of one service under one contract at a cost centre, from a day on, read from every professional's agenda (or the one named), each with the reference visoft.appointments.create takes. A professional whose agenda SIIPS did not answer is failed:true: their slots are unknown, never none. Never cached. This read never books anything.
    pub async fn lookup(&self, input: &types::VisoftAvailabilityLookupInput) -> Result<types::VisoftAvailabilityLookupOutput, Error> {
        self.0.call(&operations::VISOFT_AVAILABILITY_LOOKUP, input).await
    }
}

/// `visoft.clinic`
#[derive(Debug, Clone, Copy)]
pub struct VisoftClinicApi<'a>(&'a Client);

impl<'a> VisoftClinicApi<'a> {
    /// Get the clinic
    ///
    /// The clinic a connection signs in to: its name, its SIIPS address, its timezone (every startsAt the clinical operations answer carries its offset) and its cost centres, with the one a call that names none runs at. Another organization's connection answers CONNECTION_NOT_FOUND. Contains no patient data and performs no clinical login.
    pub async fn get(&self, input: &types::VisoftClinicGetInput) -> Result<types::VisoftClinicGetOutput, Error> {
        self.0.call(&operations::VISOFT_CLINIC_GET, input).await
    }
}

/// `visoft.clinics`
#[derive(Debug, Clone, Copy)]
pub struct VisoftClinicsApi<'a>(&'a Client);

impl<'a> VisoftClinicsApi<'a> {
    /// List the Visoft clinics
    ///
    /// The clinics the caller's organization may save a Visoft connection for, with the SIIPS address each one signs in to, in ascending id order. Another organization's clinic is never listed; an organization Arbol has not enabled yet gets none.
    pub async fn list(&self, input: &types::VisoftClinicsListInput) -> Result<types::VisoftClinicsListOutput, Error> {
        self.0.call(&operations::VISOFT_CLINICS_LIST, input).await
    }
}

/// `visoft.connections`
#[derive(Debug, Clone, Copy)]
pub struct VisoftConnectionsApi<'a>(&'a Client);

impl<'a> VisoftConnectionsApi<'a> {
    /// Remove a Visoft connection
    ///
    /// Remove what Visoft holds for one of the organization's connections, by its connectionId: its record, its SIIPS session and its cache. Account calls this before it forgets the connection. A connection Visoft holds nothing for answers CONNECTION_NOT_FOUND.
    pub async fn delete(&self, input: &types::VisoftConnectionsDeleteInput) -> Result<types::VisoftConnectionsDeleteOutput, Error> {
        self.0.call(&operations::VISOFT_CONNECTIONS_DELETE, input).await
    }

    /// Get a Visoft connection
    ///
    /// One of the organization's Visoft connections, by its connectionId: its status, its clinic, the masked username and when SIIPS last accepted its login. Never a credential. A connection the organization does not have answers CONNECTION_NOT_FOUND.
    pub async fn get(&self, input: &types::VisoftConnectionsGetInput) -> Result<types::VisoftConnectionsGetOutput, Error> {
        self.0.call(&operations::VISOFT_CONNECTIONS_GET, input).await
    }

    /// List the Visoft connections
    ///
    /// The organization's Visoft connections, as Account lists them, in ascending id order, each with its status, its clinic, the masked username and when SIIPS last accepted its login. Never a credential.
    pub async fn list(&self, input: &types::VisoftConnectionsListInput) -> Result<types::VisoftConnectionsListOutput, Error> {
        self.0.call(&operations::VISOFT_CONNECTIONS_LIST, input).await
    }

    /// Verify a Visoft connection
    ///
    /// Prove a connection's SIIPS login before Account stores it: the clinic and the username and password its team signs in to SIIPS with. SIIPS is asked once, with a real login, and the session it opens serves the connection's next calls; the same call repeated while that session lives reuses it. A refused login answers status rejected and stores nothing; SIIPS locks a user after three failed sign-ins, so a refused username is not posted again for fifteen minutes. A SIIPS user saved in another of the organization's connections is refused (INVALID_INPUT).
    pub async fn verify(&self, input: &types::VisoftConnectionsVerifyInput) -> Result<types::VisoftConnectionsVerifyOutput, Error> {
        self.0.call(&operations::VISOFT_CONNECTIONS_VERIFY, input).await
    }
}

/// `visoft.contracts`
#[derive(Debug, Clone, Copy)]
pub struct VisoftContractsApi<'a>(&'a Client);

impl<'a> VisoftContractsApi<'a> {
    /// List the clinic's contracts
    ///
    /// The clinic's current contracts (convenios) at one of its cost centres, in ascending code order, as SIIPS lists them. Cached for 12 hours; refresh reads SIIPS again.
    pub async fn list(&self, input: &types::VisoftContractsListInput) -> Result<types::VisoftContractsListOutput, Error> {
        self.0.call(&operations::VISOFT_CONTRACTS_LIST, input).await
    }
}

/// `visoft.patients`
#[derive(Debug, Clone, Copy)]
pub struct VisoftPatientsApi<'a>(&'a Client);

impl<'a> VisoftPatientsApi<'a> {
    /// Correct a patient's details
    ///
    /// Change only the fields named in the call; every other field keeps its stored value (gender identity and sexual orientation included). The change is verified by a fresh read before it is answered. PATIENT_NOT_FOUND: SIIPS has no such patient. PATIENT_NOT_SAVED: SIIPS answered but the fresh read did not show the change. A correction whose outcome is unknown answers OUTCOME_UNKNOWN and is never repeated automatically.
    pub async fn correct(&self, input: &types::VisoftPatientsCorrectInput) -> Result<types::VisoftPatientsCorrectOutput, Error> {
        self.0.call(&operations::VISOFT_PATIENTS_CORRECT, input).await
    }

    /// Register a patient
    ///
    /// Register a patient in the clinic's SIIPS at one of its cost centres, then read the record back to prove it saved. A patient SIIPS already has is answered with alreadyExisted:true and nothing written. PATIENT_NOT_SAVED: SIIPS answered but a fresh read did not show the record; read before trying again. A registration whose outcome is unknown answers OUTCOME_UNKNOWN and is never repeated automatically.
    pub async fn create(&self, input: &types::VisoftPatientsCreateInput) -> Result<types::VisoftPatientsCreateOutput, Error> {
        self.0.call(&operations::VISOFT_PATIENTS_CREATE, input).await
    }

    /// Look up a patient
    ///
    /// SIIPS's record of the patient with that document number, and, when history asks for it, the patient's agenda (each appointment with the reference visoft.appointments.cancel takes) and admissions. found:false is an answer, not a failure. The record is cached for five minutes (refresh reads SIIPS again); the agenda is read for every call.
    pub async fn lookup(&self, input: &types::VisoftPatientsLookupInput) -> Result<types::VisoftPatientsLookupOutput, Error> {
        self.0.call(&operations::VISOFT_PATIENTS_LOOKUP, input).await
    }
}

/// `visoft.professionals`
#[derive(Debug, Clone, Copy)]
pub struct VisoftProfessionalsApi<'a>(&'a Client);

impl<'a> VisoftProfessionalsApi<'a> {
    /// List the professionals of a service
    ///
    /// The professionals who attend one service at one of the clinic's cost centres, in ascending id order, as SIIPS's directory lists them. Cached for one hour; refresh reads SIIPS again.
    pub async fn list(&self, input: &types::VisoftProfessionalsListInput) -> Result<types::VisoftProfessionalsListOutput, Error> {
        self.0.call(&operations::VISOFT_PROFESSIONALS_LIST, input).await
    }
}

/// `visoft.serviceCatalogs`
#[derive(Debug, Clone, Copy)]
pub struct VisoftServiceCatalogsApi<'a>(&'a Client);

impl<'a> VisoftServiceCatalogsApi<'a> {
    /// Get a contract's service catalog
    ///
    /// Every service of one of the clinic's contracts at a cost centre, in one answer, as SIIPS lists them: for a backend that searches the whole catalog itself. A contract may list hundreds of services; people and agents page with visoft.services.list instead. Cached for 6 hours; refresh reads SIIPS again.
    pub async fn get(&self, input: &types::VisoftServiceCatalogsGetInput) -> Result<types::VisoftServiceCatalogsGetOutput, Error> {
        self.0.call(&operations::VISOFT_SERVICE_CATALOGS_GET, input).await
    }
}

/// `visoft.services`
#[derive(Debug, Clone, Copy)]
pub struct VisoftServicesApi<'a>(&'a Client);

impl<'a> VisoftServicesApi<'a> {
    /// List a contract's services
    ///
    /// The services one of the clinic's contracts covers at a cost centre, in ascending code order; q keeps those whose code or name contains each of its words, ignoring case and accents. Cached for 6 hours; refresh reads SIIPS again. A service in the catalog is not a bookable slot: visoft.availability.lookup says when one is free.
    pub async fn list(&self, input: &types::VisoftServicesListInput) -> Result<types::VisoftServicesListOutput, Error> {
        self.0.call(&operations::VISOFT_SERVICES_LIST, input).await
    }
}

/// `visoft`
#[derive(Debug, Clone, Copy)]
pub struct VisoftApi<'a>(&'a Client);

impl<'a> VisoftApi<'a> {
    pub fn appointments(&self) -> VisoftAppointmentsApi<'a> {
        VisoftAppointmentsApi(self.0)
    }

    pub fn availability(&self) -> VisoftAvailabilityApi<'a> {
        VisoftAvailabilityApi(self.0)
    }

    pub fn clinic(&self) -> VisoftClinicApi<'a> {
        VisoftClinicApi(self.0)
    }

    pub fn clinics(&self) -> VisoftClinicsApi<'a> {
        VisoftClinicsApi(self.0)
    }

    pub fn connections(&self) -> VisoftConnectionsApi<'a> {
        VisoftConnectionsApi(self.0)
    }

    pub fn contracts(&self) -> VisoftContractsApi<'a> {
        VisoftContractsApi(self.0)
    }

    pub fn patients(&self) -> VisoftPatientsApi<'a> {
        VisoftPatientsApi(self.0)
    }

    pub fn professionals(&self) -> VisoftProfessionalsApi<'a> {
        VisoftProfessionalsApi(self.0)
    }

    pub fn service_catalogs(&self) -> VisoftServiceCatalogsApi<'a> {
        VisoftServiceCatalogsApi(self.0)
    }

    pub fn services(&self) -> VisoftServicesApi<'a> {
        VisoftServicesApi(self.0)
    }
}
