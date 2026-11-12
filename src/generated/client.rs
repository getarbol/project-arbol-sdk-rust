// Generated from the pinned Arbol release by Cloudflare Forge. Do not edit.
#![allow(clippy::all, clippy::pedantic)]

use crate::generated::{operations, types};
use crate::{Client, Error};

impl Client {
    pub fn account(&self) -> AccountApi<'_> {
        AccountApi(self)
    }

    pub fn adres(&self) -> AdresApi<'_> {
        AdresApi(self)
    }

    pub fn agenda(&self) -> AgendaApi<'_> {
        AgendaApi(self)
    }

    pub fn axxa(&self) -> AxxaApi<'_> {
        AxxaApi(self)
    }

    pub fn comfachoco(&self) -> ComfachocoApi<'_> {
        ComfachocoApi(self)
    }

    pub fn gmedic(&self) -> GmedicApi<'_> {
        GmedicApi(self)
    }

    pub fn insights(&self) -> InsightsApi<'_> {
        InsightsApi(self)
    }

    pub fn nueva_eps(&self) -> NuevaEpsApi<'_> {
        NuevaEpsApi(self)
    }

    pub fn oficlinic(&self) -> OficlinicApi<'_> {
        OficlinicApi(self)
    }

    pub fn policia_colombia(&self) -> PoliciaColombiaApi<'_> {
        PoliciaColombiaApi(self)
    }

    pub fn raiz(&self) -> RaizApi<'_> {
        RaizApi(self)
    }

    pub fn salud_total(&self) -> SaludTotalApi<'_> {
        SaludTotalApi(self)
    }

    pub fn sanitas(&self) -> SanitasApi<'_> {
        SanitasApi(self)
    }

    pub fn sisben(&self) -> SisbenApi<'_> {
        SisbenApi(self)
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

/// `account.colombianRecords`
#[derive(Debug, Clone, Copy)]
pub struct AccountColombianRecordsApi<'a>(&'a Client);

impl<'a> AccountColombianRecordsApi<'a> {
    /// Consult an included Colombian source
    ///
    /// Compatibility operation. For new integrations use the dedicated ADRES, Sisbén and Policía Colombia lookups. Read an EPS summary or Sisbén classification for an authorized document. Police reports that a judicial check was not performed. An optional document labels that message and is never submitted to Police. A machine key needs connection read access and an explicit restriction allowing this operation. Set agentMode to true for concise Spanish text.
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

/// `account.registryInstallations`
#[derive(Debug, Clone, Copy)]
pub struct AccountRegistryInstallationsApi<'a>(&'a Client);

impl<'a> AccountRegistryInstallationsApi<'a> {
    /// Set the installation of a Colombian registry
    ///
    /// Install or disable one Colombian registry in your organization. Agents also need an assignment to its tool.
    pub async fn update(&self, input: &types::AccountRegistryInstallationsUpdateInput) -> Result<types::AccountRegistryInstallationsUpdateOutput, Error> {
        self.0.call(&operations::ACCOUNT_REGISTRY_INSTALLATIONS_UPDATE, input).await
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

/// `account`
#[derive(Debug, Clone, Copy)]
pub struct AccountApi<'a>(&'a Client);

impl<'a> AccountApi<'a> {
    pub fn api_keys(&self) -> AccountApiKeysApi<'a> {
        AccountApiKeysApi(self.0)
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

    pub fn members(&self) -> AccountMembersApi<'a> {
        AccountMembersApi(self.0)
    }

    pub fn organization(&self) -> AccountOrganizationApi<'a> {
        AccountOrganizationApi(self.0)
    }

    pub fn organizations(&self) -> AccountOrganizationsApi<'a> {
        AccountOrganizationsApi(self.0)
    }

    pub fn permissions(&self) -> AccountPermissionsApi<'a> {
        AccountPermissionsApi(self.0)
    }

    pub fn registry_installations(&self) -> AccountRegistryInstallationsApi<'a> {
        AccountRegistryInstallationsApi(self.0)
    }

    pub fn roles(&self) -> AccountRolesApi<'a> {
        AccountRolesApi(self.0)
    }
}

/// `adres.affiliations`
#[derive(Debug, Clone, Copy)]
pub struct AdresAffiliationsApi<'a>(&'a Client);

impl<'a> AdresAffiliationsApi<'a> {
    /// Look up EPS affiliation
    ///
    /// Read the person and all reported EPS affiliations for a document. A last-known observation is dated and does not establish current coverage.
    pub async fn lookup(&self, input: &types::AdresAffiliationsLookupInput) -> Result<types::AdresAffiliationsLookupOutput, Error> {
        self.0.call(&operations::ADRES_AFFILIATIONS_LOOKUP, input).await
    }
}

/// `adres`
#[derive(Debug, Clone, Copy)]
pub struct AdresApi<'a>(&'a Client);

impl<'a> AdresApi<'a> {
    pub fn affiliations(&self) -> AdresAffiliationsApi<'a> {
        AdresAffiliationsApi(self.0)
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
    /// Create Waitlist Entry
    ///
    /// An authenticated staff user records a consented waitlist request without sending or queuing messages, emails, offers or patient follow-ups. A repeat for the same patient, service and reason returns the existing open entry without changing it; use update with expectedVersion to amend it. Read the returned entry before claiming any preference was saved. This operation never books an appointment. Requires a user login with agenda:waitlist:write; service tokens and native agents cannot call it.
    pub async fn create(&self, input: &types::AgendaWaitlistCreateInput) -> Result<types::AgendaWaitlistCreateOutput, Error> {
        self.0.call(&operations::AGENDA_WAITLIST_CREATE, input).await
    }

    /// Add a patient to the waiting list
    ///
    /// Put a patient on the clinic's waiting list for the team to act on, only with the person's consent. Use reason 'payer_review' instead of booking when the patient's payer (EPS) has bookingPolicy 'staff_only' in get_intake_settings (create_appointment refuses them): the team coordinates with the doctor and books it. Use reason 'priority' right after create_appointment when the patient asked for a priority (urgent) appointment: pass that appointment's id as appointmentId so the team can bring it forward. For agent calls, the platform supplies conversationUrl from the invoking conversation; omit that field. Direct callers may supply the referring conversation link so the team can reopen it. Pass the patient's name, documentNumber and documentType, phone, serviceId and the payer as the patient said it. Use resourceName for the requested professional when the Agenda id is unknown. It resolves a unique active agenda for serviceId. If using resourceId, copy agendas[].id from bookingOfferings.list verbatim; its top-level id identifies an offering. Never rewrite a prefix. Search the catalog with a short clinical term, such as neurología, then select the exact serviceId. A WAITLIST_SERVICE_UNAVAILABLE, WAITLIST_RESOURCE_UNAVAILABLE or WAITLIST_RESOURCE_AMBIGUOUS refusal changes no waiting-list entry: correct the reference and retry using the existing consent. For a professional documented in the provider system but absent from Agenda, retain the restriction explicitly in note, omit resourceName and use resourceId:null on the same manual request. This is a manual preference for the team, never permission to offer any other doctor. If the person accepts several named professionals, preserve their names and the complete time window in note rather than inventing one resource ID. Calling again for the same patient, service and reason returns the same open entry. Only a request from the same verified Raíz conversation may reconcile manual and priority as one entry; payer_review stays separate. A consented reason priority request from the same verified Raíz conversation promotes a manual entry created in that same native conversation in place to priority urgent, preserving its ID, open status and queue date. For an Agenda booking include its verified appointmentId; for an external booking omit that ID and retain the verified booking in note. Historical entries, another conversation and explicit desk priority decisions are preserved; inspect the returned priority and never claim an upgrade unless it is urgent. A later manual join never lowers or duplicates that priority request. Supplied consented preferences, note, payer, conversation and appointment association can be reconciled; omit unchanged fields to retain them. A supplied confirmed phone replaces the patient contact phone; omitted or null phone preserves it. Omitted fields preserve their values; resourceId:null clears the Agenda resource filter, but never clears a named-professional restriction recorded in note. When supplying preferences, send the complete consented date/time window; null fields clear those limits. Identical repeats do not change the version. Staff status, explicit desk priority and original queue position are preserved. A closed entry is not reopened. This registers a waiting-list request, never an appointment. Then tell the patient the clinic's team will contact them.
    pub async fn join(&self, input: &types::AgendaWaitlistJoinInput) -> Result<types::AgendaWaitlistJoinOutput, Error> {
        self.0.call(&operations::AGENDA_WAITLIST_JOIN, input).await
    }

    /// List Waitlist
    ///
    /// List the organization’s waitlist entries, a page at a time: open ones first (urgent, then the longest waiting), then closed ones, most recently changed first, up to 500 in all. Narrow them by serviceId and resourceId (the agenda an entry asked for); offeringId finds only entries from before services and agendas were chosen separately.
    pub async fn list(&self, input: &types::AgendaWaitlistListInput) -> Result<types::AgendaWaitlistListOutput, Error> {
        self.0.call(&operations::AGENDA_WAITLIST_LIST, input).await
    }

    /// Update Waitlist Entry
    ///
    /// An authenticated staff user amends one existing waitlist entry with expectedVersion, preserving its ID, patient, service and queue position. Omitted preferences, professional, priority and appointment association are preserved. Send the complete consented date/time window when replacing preferences. Use resourceId:null only to clear the Agenda resource filter and retain any external or multiple-professional restriction explicitly in note. Sends and queues no messages, emails, offers or patient follow-ups and never changes a booking. Requires a user login with agenda:waitlist:write; service tokens and native agents cannot call it.
    pub async fn update(&self, input: &types::AgendaWaitlistUpdateInput) -> Result<types::AgendaWaitlistUpdateOutput, Error> {
        self.0.call(&operations::AGENDA_WAITLIST_UPDATE, input).await
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

/// `axxa.coverage`
#[derive(Debug, Clone, Copy)]
pub struct AxxaCoverageApi<'a>(&'a Client);

impl<'a> AxxaCoverageApi<'a> {
    /// Look up coverage of a service
    ///
    /// Read whether a person's plan and this provider cover one exact service code in one service category, whether it requires an authorization and, when both cover it, the voucher quote. AXA lists each service under one category, so service_not_found there does not mean it is not covered. Generates no authorization.
    pub async fn lookup(&self, input: &types::AxxaCoverageLookupInput) -> Result<types::AxxaCoverageLookupOutput, Error> {
        self.0.call(&operations::AXXA_COVERAGE_LOOKUP, input).await
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

    pub fn coverage(&self) -> AxxaCoverageApi<'a> {
        AxxaCoverageApi(self.0)
    }

    pub fn eligibility(&self) -> AxxaEligibilityApi<'a> {
        AxxaEligibilityApi(self.0)
    }

    pub fn provider(&self) -> AxxaProviderApi<'a> {
        AxxaProviderApi(self.0)
    }
}

/// `comfachoco.affiliations`
#[derive(Debug, Clone, Copy)]
pub struct ComfachocoAffiliationsApi<'a>(&'a Client);

impl<'a> ComfachocoAffiliationsApi<'a> {
    /// Lookup affiliations
    ///
    /// Look up a citizen document in Comfachocó through an active organization connection. Returns found only for an exact source identity and not_found only for an explicit source negative. Provider failures are never evidence of non-affiliation. This does not verify the identity of the person presenting the document or determine eligibility. Set agentMode to true to receive the same observation as one concise Spanish text in the text field instead of structured fields; failures are unchanged.
    pub async fn lookup(&self, input: &types::ComfachocoAffiliationsLookupInput) -> Result<types::ComfachocoAffiliationsLookupOutput, Error> {
        self.0.call(&operations::COMFACHOCO_AFFILIATIONS_LOOKUP, input).await
    }
}

/// `comfachoco.connections`
#[derive(Debug, Clone, Copy)]
pub struct ComfachocoConnectionsApi<'a>(&'a Client);

impl<'a> ComfachocoConnectionsApi<'a> {
    /// Delete a connection
    ///
    /// Remove the provider session for this organization-owned connection. Account retires the credential separately.
    pub async fn delete(&self, input: &types::ComfachocoConnectionsDeleteInput) -> Result<types::ComfachocoConnectionsDeleteOutput, Error> {
        self.0.call(&operations::COMFACHOCO_CONNECTIONS_DELETE, input).await
    }

    /// Get a connection
    ///
    /// Read safe metadata for a Comfachocó connection owned by the caller’s organization.
    pub async fn get(&self, input: &types::ComfachocoConnectionsGetInput) -> Result<types::ComfachocoConnectionsGetOutput, Error> {
        self.0.call(&operations::COMFACHOCO_CONNECTIONS_GET, input).await
    }

    /// Verify a connection
    ///
    /// Verify a provider login once before Account seals its credentials. A refused login leaves existing connections unchanged.
    pub async fn verify(&self, input: &types::ComfachocoConnectionsVerifyInput) -> Result<types::ComfachocoConnectionsVerifyOutput, Error> {
        self.0.call(&operations::COMFACHOCO_CONNECTIONS_VERIFY, input).await
    }
}

/// `comfachoco`
#[derive(Debug, Clone, Copy)]
pub struct ComfachocoApi<'a>(&'a Client);

impl<'a> ComfachocoApi<'a> {
    pub fn affiliations(&self) -> ComfachocoAffiliationsApi<'a> {
        ComfachocoAffiliationsApi(self.0)
    }

    pub fn connections(&self) -> ComfachocoConnectionsApi<'a> {
        ComfachocoConnectionsApi(self.0)
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

/// `insights.reports`
#[derive(Debug, Clone, Copy)]
pub struct InsightsReportsApi<'a>(&'a Client);

impl<'a> InsightsReportsApi<'a> {
    /// Read a report
    ///
    /// One report over a date range, for the caller's own organization. Figures refresh within 30 minutes.
    pub async fn get(&self, input: &types::InsightsReportsGetInput) -> Result<types::InsightsReportsGetOutput, Error> {
        self.0.call(&operations::INSIGHTS_REPORTS_GET, input).await
    }

    /// List the available reports
    ///
    /// The reports this organization can read, a page at a time. Pass pageInfo.nextCursor as cursor for more.
    pub async fn list(&self, input: &types::InsightsReportsListInput) -> Result<types::InsightsReportsListOutput, Error> {
        self.0.call(&operations::INSIGHTS_REPORTS_LIST, input).await
    }
}

/// `insights`
#[derive(Debug, Clone, Copy)]
pub struct InsightsApi<'a>(&'a Client);

impl<'a> InsightsApi<'a> {
    pub fn reports(&self) -> InsightsReportsApi<'a> {
        InsightsReportsApi(self.0)
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

/// `nueva-eps.ingestionErrorReports`
#[derive(Debug, Clone, Copy)]
pub struct NuevaEpsIngestionErrorReportsApi<'a>(&'a Client);

impl<'a> NuevaEpsIngestionErrorReportsApi<'a> {
    /// Download a Precursoras error report
    ///
    /// Get a private XLSX validation report with a download link valid for 15 minutes. The report id is its upload id. May contain patient information. Files expire from storage after 24 hours. Provider failures are returned explicitly.
    pub async fn get(&self, input: &types::NuevaEpsIngestionErrorReportsGetInput) -> Result<types::NuevaEpsIngestionErrorReportsGetOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_INGESTION_ERROR_REPORTS_GET, input).await
    }
}

/// `nueva-eps.ingestions`
#[derive(Debug, Clone, Copy)]
pub struct NuevaEpsIngestionsApi<'a>(&'a Client);

impl<'a> NuevaEpsIngestionsApi<'a> {
    /// Get a Precursoras upload
    ///
    /// Read one upload’s processing status and row counts after verifying it belongs to this authenticated account’s providers and structures.
    pub async fn get(&self, input: &types::NuevaEpsIngestionsGetInput) -> Result<types::NuevaEpsIngestionsGetOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_INGESTIONS_GET, input).await
    }

    /// List Precursoras uploads
    ///
    /// Read upload processing summaries for a provider and structure from this account’s catalog. Cursors use the provider’s live page boundaries; concurrent uploads may shift them. No patient rows, file paths or clinical writes.
    pub async fn list(&self, input: &types::NuevaEpsIngestionsListInput) -> Result<types::NuevaEpsIngestionsListOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_INGESTIONS_LIST, input).await
    }
}

/// `nueva-eps.precursoras`
#[derive(Debug, Clone, Copy)]
pub struct NuevaEpsPrecursorasApi<'a>(&'a Client);

impl<'a> NuevaEpsPrecursorasApi<'a> {
    /// Get the Precursoras catalog
    ///
    /// Read this account’s available providers and reporting structures. Requires a separate Precursoras Account connection; affiliation credentials cannot be used.
    pub async fn get(&self, input: &types::NuevaEpsPrecursorasGetInput) -> Result<types::NuevaEpsPrecursorasGetOutput, Error> {
        self.0.call(&operations::NUEVA_EPS_PRECURSORAS_GET, input).await
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

    pub fn ingestion_error_reports(&self) -> NuevaEpsIngestionErrorReportsApi<'a> {
        NuevaEpsIngestionErrorReportsApi(self.0)
    }

    pub fn ingestions(&self) -> NuevaEpsIngestionsApi<'a> {
        NuevaEpsIngestionsApi(self.0)
    }

    pub fn precursoras(&self) -> NuevaEpsPrecursorasApi<'a> {
        NuevaEpsPrecursorasApi(self.0)
    }
}

/// `oficlinic.affiliateTypes`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicAffiliateTypesApi<'a>(&'a Client);

impl<'a> OficlinicAffiliateTypesApi<'a> {
    /// List affiliate types
    ///
    /// List the affiliate types e-Oficlinic offers for one regime, in the provider's order, as its booking screen loads them (cbtipoafiliado). The regime must be one list_regimes answers, or BOOKING_OPTION_NOT_FOUND. A booking takes one as classification.affiliateType, listed for its classification.regime. Which one applies to a patient is the clinic's decision, never derived here. Pass pageInfo.nextCursor with the same regimeId for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicAffiliateTypesListInput) -> Result<types::OficlinicAffiliateTypesListOutput, Error> {
        self.0.call(&operations::OFICLINIC_AFFILIATE_TYPES_LIST, input).await
    }
}

/// `oficlinic.appointments`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicAppointmentsApi<'a>(&'a Client);

impl<'a> OficlinicAppointmentsApi<'a> {
    /// Cancel an appointment
    ///
    /// Cancel one upcoming assigned e-Oficlinic appointment of the patient resolved by citizenship card. Use the exact CAS (serviceNumber) from that patient's assigned appointments, one exact listed cancellation reason, and confirm true only after the patient or authorized staff confirms this specific cancellation. The same idempotency key and request return the recorded result without sending again. OUTCOME_UNKNOWN means the cancellation may have happened: look up the patient's cancelled appointments and do not dispatch it again. agentMode true answers concise Spanish text instead.
    pub async fn cancel(&self, input: &types::OficlinicAppointmentsCancelInput) -> Result<types::OficlinicAppointmentsCancelOutput, Error> {
        self.0.call(&operations::OFICLINIC_APPOINTMENTS_CANCEL, input).await
    }

    /// Create an appointment
    ///
    /// Book one appointment in e-Oficlinic (ASIGNAR_CITAS) for a slot that list_booking_slots offered in an availability flow. Requires confirm true and an idempotencyKey. Before anything is written, the patient is resolved by citizenship card; the program and attention type must be the flow's, and the source must still count exactly one unit for them; the regime, affiliate type and relationship must be listed by the provider; the payer, plan, subplan and contract must be listed by the provider for the connection; the service must be configured for the slot's professional; and a fresh read must still show the slot. The classification, billing and referral values are the clinic's own provider codes, which the caller supplies explicitly: this service never guesses them, never derives them from a program and never loads the clinic's tariff matrix. The intent and a fence on the slot are recorded before the single provider request, which is never resent. Success means the provider answered a strictly positive appointment number, the source's own creation signal (verification provider_acknowledgement). OUTCOME_UNKNOWN means the appointment may exist: do not book again; check lookup_appointments with status assigned and ask staff. WRITE_TARGET_UNRESOLVED means another key's write to this slot has no final outcome, so nothing was sent. WRITE_LEDGER_FULL means the connection's ledger refused a new write and nothing was sent. Repeating the same idempotencyKey and request answers the recorded outcome. One-slot, one-unit bookings only. agentMode true answers concise Spanish text instead.
    pub async fn create(&self, input: &types::OficlinicAppointmentsCreateInput) -> Result<types::OficlinicAppointmentsCreateOutput, Error> {
        self.0.call(&operations::OFICLINIC_APPOINTMENTS_CREATE, input).await
    }

    /// Lookup appointments
    ///
    /// Look up one patient's e-Oficlinic appointment history by status: assigned, cancelled, absence (no-show) or legalized (attendance registered). The patient is resolved by citizenship card number first, sent in the body; PATIENT_NOT_FOUND answers only the provider explicit negative, and every row must name the resolved patient. Staff usernames and free-text observations are never returned. serviceNumber is the appointment's CAS, which reschedule_appointment takes for an assigned appointment. Read-only. Pass pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn lookup(&self, input: &types::OficlinicAppointmentsLookupInput) -> Result<types::OficlinicAppointmentsLookupOutput, Error> {
        self.0.call(&operations::OFICLINIC_APPOINTMENTS_LOOKUP, input).await
    }

    /// Reschedule an appointment
    ///
    /// Reschedule one assigned e-Oficlinic appointment. The provider has no individual reschedule (its REPROGRAMAR_AGENDA moves a professional's whole agenda and is never used), so this is a saga, never atomic: the original must be an upcoming assigned appointment of the patient resolved by citizenship card (its CAS, serviceNumber in lookup_appointments); the cancellation reason must be exactly one list_cancellation_reasons answers; the new slot is booked first exactly as create_appointment books it; only after the provider acknowledged the new booking is the original cancelled (CANECELAR_CITAS) with the given reason and observations. If the new booking fails, nothing changes; if its outcome is unknown, OUTCOME_UNKNOWN answers and the original is never cancelled. Once the new booking is acknowledged the answer reports each phase: completed when the cancellation was confirmed too, partial otherwise (cancellation pending, unknown or not possible), when the patient may hold both appointments. pending continues when the same request is repeated with the same idempotencyKey, which resumes the cancellation alone; nothing is ever resent. The slot and the original are fenced from the first dispatch: WRITE_TARGET_UNRESOLVED means another key's write to either has no final outcome, and nothing was sent. agentMode true answers concise Spanish text instead.
    pub async fn reschedule(&self, input: &types::OficlinicAppointmentsRescheduleInput) -> Result<types::OficlinicAppointmentsRescheduleOutput, Error> {
        self.0.call(&operations::OFICLINIC_APPOINTMENTS_RESCHEDULE, input).await
    }
}

/// `oficlinic.bookingDays`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicBookingDaysApi<'a>(&'a Client);

impl<'a> OficlinicBookingDaysApi<'a> {
    /// List booking days
    ///
    /// List the agenda days e-Oficlinic offers for one professional and room of the flow, for the flow's payer and service (LISTAR_AGENDAS_DISPONIBLES). The room must be one list_booking_rooms offered for that professional in the same flow. An empty list is the provider's answer for this exact combination alone, never for the whole clinic. Read-only: nothing is locked. Pass pageInfo.nextCursor with the same filters for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicBookingDaysListInput) -> Result<types::OficlinicBookingDaysListOutput, Error> {
        self.0.call(&operations::OFICLINIC_BOOKING_DAYS_LIST, input).await
    }
}

/// `oficlinic.bookingProfessionals`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicBookingProfessionalsApi<'a>(&'a Client);

impl<'a> OficlinicBookingProfessionalsApi<'a> {
    /// List booking professionals
    ///
    /// List the professionals e-Oficlinic schedules for one service (LISTAR_PROFESIONALES_AGENDA) and open an availability flow for that payer, service, program and attention type; every item carries the flowId the next steps take (list_booking_rooms, list_booking_days, list_booking_slots, then create_appointment). The flow keeps, server-side, the provider's own flow identifier its booking screen generates; no slot is locked or reserved by listing. The payer must be one list_payers answers, or PAYER_NOT_FOUND. The program and attention type come from list_programs, or BOOKING_OPTION_NOT_FOUND; an attention type whose source unit count is not exactly one answers PROGRAM_UNITS_NOT_SUPPORTED, since multi-unit programs are not supported. The flow expires 30 minutes after this first list. Use a configured service id (search_services, or list_services with professionalId). An empty list means the provider schedules no professional for that service now. Pass pageInfo.nextCursor with the same filters for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicBookingProfessionalsListInput) -> Result<types::OficlinicBookingProfessionalsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_BOOKING_PROFESSIONALS_LIST, input).await
    }
}

/// `oficlinic.bookingRooms`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicBookingRoomsApi<'a>(&'a Client);

impl<'a> OficlinicBookingRoomsApi<'a> {
    /// List booking rooms
    ///
    /// List the rooms where e-Oficlinic schedules one professional for the flow's service (LISTAR_CONSULTORIOSxPROFE_AGENDA). The professional must be one list_booking_professionals offered in the same flow, or SELECTION_NOT_AVAILABLE; an expired or unknown flow answers SELECTION_EXPIRED. Read-only: nothing is locked. Pass pageInfo.nextCursor with the same filters for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicBookingRoomsListInput) -> Result<types::OficlinicBookingRoomsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_BOOKING_ROOMS_LIST, input).await
    }
}

/// `oficlinic.bookingSlots`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicBookingSlotsApi<'a>(&'a Client);

impl<'a> OficlinicBookingSlotsApi<'a> {
    /// List booking slots
    ///
    /// List the slots e-Oficlinic shows for one agenda day of the flow, for the flow's payer and its one-unit attention type, as its booking screen shows them (LISTAR_HORARIOS_AGENDAS_DISPONIBLES with the screen's display filter). The day must be one list_booking_days offered in the same flow. startsAt and endsAt come from the source's exact time range, and are null rather than guessed when it is unrecognized. Observed live for one agenda; the rows observed had no Estado, so status is usually null. Read-only: nothing is locked or reserved. For create_appointment or reschedule_appointment, copy the selected slot's flowId and slotId verbatim (slotId is called id in the structured result). Keep the two values together and refresh availability when the flow expires. Pass pageInfo.nextCursor with the same filters for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicBookingSlotsListInput) -> Result<types::OficlinicBookingSlotsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_BOOKING_SLOTS_LIST, input).await
    }

    /// Search booking slots
    ///
    /// Find the nearest real slots e-Oficlinic offers for one payer, service, program and attention type, in one call: it opens an availability flow, reads the professionals the provider schedules for the service, their rooms and each combination's agenda days, then the slots of the earliest days, and answers up to slotsPerRoom future slots for each room with the professional, the room, the day and the flowId and slotId that book it. Use it instead of walking list_booking_professionals, list_booking_rooms, list_booking_days and list_booking_slots. Without from it starts today in Colombia; for one requested day pass that day as both from and to. professionalIds keeps a named professional; roomIds keeps the rooms of one site, since a room is the place of care and this provider names the site only in the room's name. dates lists every agenda day with availability found in those days, at most 60, so another date can be offered and searched next. Slots already past are left out. complete false means a bound stopped the search early (at most 40 professionals, 60 professional and room combinations, 16 agenda days read for slots, and 8 seconds): narrow the filters, and never present the result as the nearest. An empty slots list concerns this payer, service and filters only. The payer must be one list_payers answers, or PAYER_NOT_FOUND; the program and attention type come from list_programs, or BOOKING_OPTION_NOT_FOUND, and an attention type that is not exactly one unit answers PROGRAM_UNITS_NOT_SUPPORTED. Read-only: nothing is locked or reserved. The flow expires 30 minutes after the search; for create_appointment or reschedule_appointment, copy the selected slot's flowId and slotId verbatim. agentMode true answers concise Spanish text instead.
    pub async fn search(&self, input: &types::OficlinicBookingSlotsSearchInput) -> Result<types::OficlinicBookingSlotsSearchOutput, Error> {
        self.0.call(&operations::OFICLINIC_BOOKING_SLOTS_SEARCH, input).await
    }
}

/// `oficlinic.cancellationReasons`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicCancellationReasonsApi<'a>(&'a Client);

impl<'a> OficlinicCancellationReasonsApi<'a> {
    /// List cancellation reasons
    ///
    /// List the cancellation reasons e-Oficlinic offers (cbmotivocancelainasiste), in the provider's order. The source gives them no id: pass one exactly as cancellation.reason of reschedule_appointment, which refuses any other text with BOOKING_OPTION_NOT_FOUND. Pass pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicCancellationReasonsListInput) -> Result<types::OficlinicCancellationReasonsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_CANCELLATION_REASONS_LIST, input).await
    }
}

/// `oficlinic.connections`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicConnectionsApi<'a>(&'a Client);

impl<'a> OficlinicConnectionsApi<'a> {
    /// Delete a connection
    ///
    /// Invalidate the provider session of an e-Oficlinic connection of the caller's organization: its cookies and session state are forgotten at once, even during a read. Account retires the sealed credentials separately. Repeating the call answers the same.
    pub async fn delete(&self, input: &types::OficlinicConnectionsDeleteInput) -> Result<types::OficlinicConnectionsDeleteOutput, Error> {
        self.0.call(&operations::OFICLINIC_CONNECTIONS_DELETE, input).await
    }

    /// Get a connection
    ///
    /// Read an e-Oficlinic connection of the caller's organization: Account's name and status, the provider session's state and why it stopped, the selected company, site and IPS, and when it was verified and last used. It never contacts the provider and never returns credentials or cookies.
    pub async fn get(&self, input: &types::OficlinicConnectionsGetInput) -> Result<types::OficlinicConnectionsGetOutput, Error> {
        self.0.call(&operations::OFICLINIC_CONNECTIONS_GET, input).await
    }
}

/// `oficlinic.dailyRoster`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicDailyRosterApi<'a>(&'a Client);

impl<'a> OficlinicDailyRosterApi<'a> {
    /// Get daily roster
    ///
    /// Read the e-Oficlinic daily patient roster for one professional and one date in Colombia. For authorized clinic staff only. Returns scheduled time, appointment number, patient display name, service and location. A vacant slot is not an appointment. Use the professional's id from list_professionals. Page through the same professional and date with pageInfo.nextCursor; results do not cover other professionals or sites.
    pub async fn list(&self, input: &types::OficlinicDailyRosterListInput) -> Result<types::OficlinicDailyRosterListOutput, Error> {
        self.0.call(&operations::OFICLINIC_DAILY_ROSTER_LIST, input).await
    }
}

/// `oficlinic.patientRegistrationOptions`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicPatientRegistrationOptionsApi<'a>(&'a Client);

impl<'a> OficlinicPatientRegistrationOptionsApi<'a> {
    /// List patient registration options
    ///
    /// Find the source codes for a patient's confirmed gender, residence zone or municipality before creating their clinic record. q filters the current provider catalog by name without accents, or exact code; use it for municipalities to keep the result short. A choice is not a fact about the patient: ask for missing profile details and never invent them. For regime, affiliate type and salary range use their dedicated lists. Pass pageInfo.nextCursor with the same kind and q for more. Read-only. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicPatientRegistrationOptionsListInput) -> Result<types::OficlinicPatientRegistrationOptionsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_PATIENT_REGISTRATION_OPTIONS_LIST, input).await
    }
}

/// `oficlinic.patients`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicPatientsApi<'a>(&'a Client);

impl<'a> OficlinicPatientsApi<'a> {
    /// Create a patient
    ///
    /// Create one new patient in e-Oficlinic using the clinic-confirmed legal identity, contact and classification fields. Use only with verified authority to register this person and explicit confirmation of their details. Requires confirm true and one idempotency key for this intent. An existing patient is never changed. An uncertain result must be checked with lookup_patient and must not be sent again. The result contains no patient data. Verify the saved patient with lookup_patient before attempting a booking. The submission follows the live-verified Cardiología scheduling form. agentMode true answers concise Spanish text instead.
    pub async fn create(&self, input: &types::OficlinicPatientsCreateInput) -> Result<types::OficlinicPatientsCreateOutput, Error> {
        self.0.call(&operations::OFICLINIC_PATIENTS_CREATE, input).await
    }

    /// Look up a patient by document
    ///
    /// Look up one patient in e-Oficlinic by citizenship card (CC) number, sent in the body and never in a URL. found answers a deliberately projected record (provider id, names, birth date, sex, address, phones, email, municipality) only when the provider returned a record with the requested document; not_found answers only the provider explicit negative. Any failure is not evidence that the patient is absent. This does not verify payer rights or the identity of the person presenting the document. agentMode true answers concise Spanish text instead.
    pub async fn lookup(&self, input: &types::OficlinicPatientsLookupInput) -> Result<types::OficlinicPatientsLookupOutput, Error> {
        self.0.call(&operations::OFICLINIC_PATIENTS_LOOKUP, input).await
    }
}

/// `oficlinic.payerContracts`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicPayerContractsApi<'a>(&'a Client);

impl<'a> OficlinicPayerContractsApi<'a> {
    /// List payer contracts
    ///
    /// List the contracts e-Oficlinic holds for one payer of the connection, in the provider's order, as its booking screen loads them (cbclientescontrato): id, names, number, validity days and active flag. Financial and internal contract fields are never returned. The payer must be one list_payers answers, or PAYER_NOT_FOUND. A contract is not proof of coverage for a patient. Pass pageInfo.nextCursor with the same payerId for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicPayerContractsListInput) -> Result<types::OficlinicPayerContractsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_PAYER_CONTRACTS_LIST, input).await
    }
}

/// `oficlinic.payerPlans`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicPayerPlansApi<'a>(&'a Client);

impl<'a> OficlinicPayerPlansApi<'a> {
    /// List payer plans
    ///
    /// List the plans e-Oficlinic offers for one payer of the connection, in the provider's order, as its booking screen loads them (cbclientesplan). The payer must be one list_payers answers for the connection, or PAYER_NOT_FOUND. A booking takes one of these plans. Pass pageInfo.nextCursor with the same payerId for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicPayerPlansListInput) -> Result<types::OficlinicPayerPlansListOutput, Error> {
        self.0.call(&operations::OFICLINIC_PAYER_PLANS_LIST, input).await
    }
}

/// `oficlinic.payerSubplans`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicPayerSubplansApi<'a>(&'a Client);

impl<'a> OficlinicPayerSubplansApi<'a> {
    /// List payer subplans
    ///
    /// List the subplans e-Oficlinic offers for one plan of a payer, in the provider's order, as its booking screen loads them (cbclientessubplan). The payer must be the connection's and the plan one of the payer's (list_payer_plans), or PAYER_NOT_FOUND or PAYER_OPTION_NOT_FOUND. A booking takes one of these subplans. Pass pageInfo.nextCursor with the same filters for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicPayerSubplansListInput) -> Result<types::OficlinicPayerSubplansListOutput, Error> {
        self.0.call(&operations::OFICLINIC_PAYER_SUBPLANS_LIST, input).await
    }
}

/// `oficlinic.payers`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicPayersApi<'a>(&'a Client);

impl<'a> OficlinicPayersApi<'a> {
    /// List payers
    ///
    /// List the payers (EPS, insurers and other contracting clients) configured in e-Oficlinic for the connection's verified company and IPS, in the provider's order. Flags the provider does not spell in a recognized way are null. Pass connectionId when the organization has several active connections, and pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicPayersListInput) -> Result<types::OficlinicPayersListOutput, Error> {
        self.0.call(&operations::OFICLINIC_PAYERS_LIST, input).await
    }
}

/// `oficlinic.professionals`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicProfessionalsApi<'a>(&'a Client);

impl<'a> OficlinicProfessionalsApi<'a> {
    /// List professionals
    ///
    /// List the professionals of the e-Oficlinic account's directory, a page at a time in the provider's order: id, name, specialty and active flag. Identity documents, addresses and phones of professionals are never returned. This directory is not an availability or scheduling answer: list_booking_professionals answers who the provider schedules for a service. Pass pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicProfessionalsListInput) -> Result<types::OficlinicProfessionalsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_PROFESSIONALS_LIST, input).await
    }
}

/// `oficlinic.programs`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicProgramsApi<'a>(&'a Client);

impl<'a> OficlinicProgramsApi<'a> {
    /// List programs
    ///
    /// List the programs e-Oficlinic offers on its booking screen (cbprograma), each with the attention types it offers (cbprogramatipoatencion), in the provider's order. Opening an availability flow and booking take a program and one of its attention types whose bookable is true: the source counts exactly one unit for it. Multi-unit programs are listed and not bookable here, and a missing or unrecognized count is null, never assumed to be one. sourcePurpose and exempt are catalog facts: they decide nothing about a booking, whose purposeId and billing values stay explicit. Pass pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicProgramsListInput) -> Result<types::OficlinicProgramsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_PROGRAMS_LIST, input).await
    }
}

/// `oficlinic.regimes`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicRegimesApi<'a>(&'a Client);

impl<'a> OficlinicRegimesApi<'a> {
    /// List regimes
    ///
    /// List the regimes e-Oficlinic offers on its booking screen (cbtipousuario), in the provider's order. A booking takes one as classification.regime, and list_affiliate_types takes its id as regimeId. Which regime applies to a patient is the clinic's decision, never derived here. Pass pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicRegimesListInput) -> Result<types::OficlinicRegimesListOutput, Error> {
        self.0.call(&operations::OFICLINIC_REGIMES_LIST, input).await
    }
}

/// `oficlinic.relationships`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicRelationshipsApi<'a>(&'a Client);

impl<'a> OficlinicRelationshipsApi<'a> {
    /// List relationships
    ///
    /// List the relationships to the policy holder e-Oficlinic offers on its booking screen (cbparentesco), in the provider's order. A booking may take one as classification.relationship. Which one applies to a patient is the clinic's decision, never derived here. Pass pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicRelationshipsListInput) -> Result<types::OficlinicRelationshipsListOutput, Error> {
        self.0.call(&operations::OFICLINIC_RELATIONSHIPS_LIST, input).await
    }
}

/// `oficlinic.salaryRanges`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicSalaryRangesApi<'a>(&'a Client);

impl<'a> OficlinicSalaryRangesApi<'a> {
    /// List salary ranges
    ///
    /// List the salary ranges that the e-Oficlinic registration form offers for one listed regime. The provider's code can identify a clinic-confirmed classification for scheduling. The catalog amount does not establish the patient's classification, service tariff or amount due. Pass pageInfo.nextCursor with the same regimeId for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicSalaryRangesListInput) -> Result<types::OficlinicSalaryRangesListOutput, Error> {
        self.0.call(&operations::OFICLINIC_SALARY_RANGES_LIST, input).await
    }
}

/// `oficlinic.serviceCatalog`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicServiceCatalogApi<'a>(&'a Client);

impl<'a> OficlinicServiceCatalogApi<'a> {
    /// Get the service catalog index
    ///
    /// Read the state of the connection's local index of the e-Oficlinic service catalog that search_services ranks: whether a complete generation serves searches and when it was completed, its counts, and the progress or last failure of its sync. It never contacts the provider. agentMode true answers concise Spanish text instead.
    pub async fn get(&self, input: &types::OficlinicServiceCatalogGetInput) -> Result<types::OficlinicServiceCatalogGetOutput, Error> {
        self.0.call(&operations::OFICLINIC_SERVICE_CATALOG_GET, input).await
    }

    /// Sync the service catalog index
    ///
    /// Start building a new generation of the connection's local service index from the provider's complete master catalog, professional directory and active professionals' configured services, or resume the one building. It reads in bounded batches between other calls, checks that the provider's totals stay constant and every id appears once, and publishes the generation only when complete; until then the previous generation keeps serving searches. A sync started in the last ten minutes that did not fail is not repeated. Answers the index's state.
    pub async fn sync(&self, input: &types::OficlinicServiceCatalogSyncInput) -> Result<types::OficlinicServiceCatalogSyncOutput, Error> {
        self.0.call(&operations::OFICLINIC_SERVICE_CATALOG_SYNC, input).await
    }
}

/// `oficlinic.services`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicServicesApi<'a>(&'a Client);

impl<'a> OficlinicServicesApi<'a> {
    /// List services
    ///
    /// List the e-Oficlinic master service catalog (procedures and studies with their CUPS codes), a page at a time in the provider's order. It lists what the provider can register, about thirteen thousand entries: it is not contracted coverage, a payer authorization or a clinical recommendation. With professionalId, it lists instead the services the provider configured for that professional (GRID_PROCEDIMIENTOS), read live: configuration, not coverage or open slots, and its cupsDescription, specialPreparation and active are null. To find a service by text, use search_services. Pass pageInfo.nextCursor as cursor for more. agentMode true answers concise Spanish text instead.
    pub async fn list(&self, input: &types::OficlinicServicesListInput) -> Result<types::OficlinicServicesListOutput, Error> {
        self.0.call(&operations::OFICLINIC_SERVICES_LIST, input).await
    }

    /// Search services
    ///
    /// Search the e-Oficlinic services by text over the connection's local index of the provider's complete master catalog and its professionals' configured services (GRID_PROCEDIMIENTOS), read through the connection and published only when complete. Results rank by an exact provider id, code or CUPS code, then the text as a phrase of the name, every word (accents ignored, prefixes allowed), then words within a small spelling distance. excludeTerms and excludeServiceIds are hard exclusions applied before ranking: no excluded service ever comes back, related ones included. configuredOnly (true by default) keeps services configured for at least one active professional; false searches the whole master catalog, which is not coverage. includeRelated adds, below every direct match, services configured for active professionals whose specialty matches the text, marked related_specialty: it is not a claim of clinical equivalence. Nothing here is coverage, an authorization or an open slot. The first search of a connection starts building the index and answers INDEX_NOT_READY until it is complete; an older index keeps serving while a refresh builds. Pass pageInfo.nextCursor with the same filters for more. agentMode true answers concise Spanish text instead.
    pub async fn search(&self, input: &types::OficlinicServicesSearchInput) -> Result<types::OficlinicServicesSearchOutput, Error> {
        self.0.call(&operations::OFICLINIC_SERVICES_SEARCH, input).await
    }
}

/// `oficlinic`
#[derive(Debug, Clone, Copy)]
pub struct OficlinicApi<'a>(&'a Client);

impl<'a> OficlinicApi<'a> {
    pub fn affiliate_types(&self) -> OficlinicAffiliateTypesApi<'a> {
        OficlinicAffiliateTypesApi(self.0)
    }

    pub fn appointments(&self) -> OficlinicAppointmentsApi<'a> {
        OficlinicAppointmentsApi(self.0)
    }

    pub fn booking_days(&self) -> OficlinicBookingDaysApi<'a> {
        OficlinicBookingDaysApi(self.0)
    }

    pub fn booking_professionals(&self) -> OficlinicBookingProfessionalsApi<'a> {
        OficlinicBookingProfessionalsApi(self.0)
    }

    pub fn booking_rooms(&self) -> OficlinicBookingRoomsApi<'a> {
        OficlinicBookingRoomsApi(self.0)
    }

    pub fn booking_slots(&self) -> OficlinicBookingSlotsApi<'a> {
        OficlinicBookingSlotsApi(self.0)
    }

    pub fn cancellation_reasons(&self) -> OficlinicCancellationReasonsApi<'a> {
        OficlinicCancellationReasonsApi(self.0)
    }

    pub fn connections(&self) -> OficlinicConnectionsApi<'a> {
        OficlinicConnectionsApi(self.0)
    }

    pub fn daily_roster(&self) -> OficlinicDailyRosterApi<'a> {
        OficlinicDailyRosterApi(self.0)
    }

    pub fn patient_registration_options(&self) -> OficlinicPatientRegistrationOptionsApi<'a> {
        OficlinicPatientRegistrationOptionsApi(self.0)
    }

    pub fn patients(&self) -> OficlinicPatientsApi<'a> {
        OficlinicPatientsApi(self.0)
    }

    pub fn payer_contracts(&self) -> OficlinicPayerContractsApi<'a> {
        OficlinicPayerContractsApi(self.0)
    }

    pub fn payer_plans(&self) -> OficlinicPayerPlansApi<'a> {
        OficlinicPayerPlansApi(self.0)
    }

    pub fn payer_subplans(&self) -> OficlinicPayerSubplansApi<'a> {
        OficlinicPayerSubplansApi(self.0)
    }

    pub fn payers(&self) -> OficlinicPayersApi<'a> {
        OficlinicPayersApi(self.0)
    }

    pub fn professionals(&self) -> OficlinicProfessionalsApi<'a> {
        OficlinicProfessionalsApi(self.0)
    }

    pub fn programs(&self) -> OficlinicProgramsApi<'a> {
        OficlinicProgramsApi(self.0)
    }

    pub fn regimes(&self) -> OficlinicRegimesApi<'a> {
        OficlinicRegimesApi(self.0)
    }

    pub fn relationships(&self) -> OficlinicRelationshipsApi<'a> {
        OficlinicRelationshipsApi(self.0)
    }

    pub fn salary_ranges(&self) -> OficlinicSalaryRangesApi<'a> {
        OficlinicSalaryRangesApi(self.0)
    }

    pub fn service_catalog(&self) -> OficlinicServiceCatalogApi<'a> {
        OficlinicServiceCatalogApi(self.0)
    }

    pub fn services(&self) -> OficlinicServicesApi<'a> {
        OficlinicServicesApi(self.0)
    }
}

/// `policia-colombia.judicialRecords`
#[derive(Debug, Clone, Copy)]
pub struct PoliciaColombiaJudicialRecordsApi<'a>(&'a Client);

impl<'a> PoliciaColombiaJudicialRecordsApi<'a> {
    /// Look up a Colombian judicial statement
    ///
    /// Read the document-matched identity fields and verbatim source statement, when available. A statement about pending matters is not a criminal conviction history.
    pub async fn lookup(&self, input: &types::PoliciaColombiaJudicialRecordsLookupInput) -> Result<types::PoliciaColombiaJudicialRecordsLookupOutput, Error> {
        self.0.call(&operations::POLICIA_COLOMBIA_JUDICIAL_RECORDS_LOOKUP, input).await
    }
}

/// `policia-colombia`
#[derive(Debug, Clone, Copy)]
pub struct PoliciaColombiaApi<'a>(&'a Client);

impl<'a> PoliciaColombiaApi<'a> {
    pub fn judicial_records(&self) -> PoliciaColombiaJudicialRecordsApi<'a> {
        PoliciaColombiaJudicialRecordsApi(self.0)
    }
}

/// `raiz.agents`
#[derive(Debug, Clone, Copy)]
pub struct RaizAgentsApi<'a>(&'a Client);

impl<'a> RaizAgentsApi<'a> {
    /// Create an Agent
    ///
    /// Create an Agent that talks to the organization's contacts. Choose `modality` (`voice`: phone calls; `text`: WhatsApp and web chat) and a `name`; then give it `instructions` (role, goal, limits), and optionally a `firstMessage`, `language`, `timeZone`, `settings`, Knowledge (`knowledgeDocumentIds` of ready documents from `list_knowledge_documents`) and `integrationTools` (connection + tool pairs of the organization's connections in Account, checked live: INTEGRATION_TOOL_UNAVAILABLE), `procedures` (each a trigger plus free-form Markdown or structured steps, with inline references to the Agent's tools, Knowledge and other procedures: AGENT_PROCEDURES_INVALID says where one is wrong), `workflow` (a conversation graph of steps and edges: AGENT_WORKFLOW_INVALID says where one is wrong), and for a text Agent `integrationProcedures` (connection + procedure pairs a connection publishes, checked live: INTEGRATION_PROCEDURE_UNAVAILABLE). Next: assign it to a channel with `update_channel`; a voice Agent is published in the background, so check `synced` with `get_agent`. Read the `create-agent` skill (`get_skill`) first.
    pub async fn create(&self, input: &types::RaizAgentsCreateInput) -> Result<types::RaizAgentsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_AGENTS_CREATE, input).await
    }

    /// Delete an Agent
    ///
    /// Delete an Agent for good; its conversations remain. Refused with AGENT_IN_USE while a channel or an active campaign uses it: unassign its channels (`update_channel` with `agentId: null`) or finish the campaign first. Not an AI tool; to stop an Agent without deleting it, pause it with `update_agent` (`status: paused`).
    pub async fn delete(&self, input: &types::RaizAgentsDeleteInput) -> Result<types::RaizAgentsDeleteOutput, Error> {
        self.0.call(&operations::RAIZ_AGENTS_DELETE, input).await
    }

    /// Get an Agent
    ///
    /// Read one Agent's complete setup: instructions, first message, language, time zone, settings, integration tools (each with its connection and effect), Knowledge document ids, procedures (trigger plus free-form instructions or structured steps) and connection procedures, its workflow (a conversation graph, or null) and channels. `ready` says whether it can hold conversations; for a voice Agent, `synced` says whether its phone line already runs this version, and `syncErrorCode` why not. Read it before `update_agent` and send its `version` as `expectedVersion`.
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
    /// Change an Agent: instructions, first message, name, language, time zone, settings, integration tools, Knowledge, procedures, connection procedures, workflow or status. Omitted fields stay and `null` clears one, also inside `settings`; `integrationTools`, `integrationProcedures`, `procedures`, `knowledgeDocumentIds` and `workflow` replace the whole value when given (keep each existing procedure's `id`), so read the Agent with `get_agent` first and send its `version` as `expectedVersion` (VERSION_CONFLICT: read it again). A workflow is checked against the tools the Agent will have (AGENT_WORKFLOW_INVALID). `status: paused` stops it from answering and calling. `modality` changes only while it has no channels and no published voice line (AGENT_MODALITY_LOCKED). A voice Agent republishes in the background: check `synced`.
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
    /// Create a campaign
    ///
    /// Prepare a campaign with up to 10000 recipients, each a phone `address` with an optional `contactId` and `variables`. A call campaign (the default) has a voice Agent call each one. With `channelType: whatsapp` and an approved `template`, each one is sent the template, its `parameters` filled from their `variables` (`{{name}}`) and, when its header is a document, image or video, their own `header` file; its text Agent answers whoever replies. Nobody is reached until it is started. A list adds nobody to the organization's contacts: a recipient becomes the contact it names, or the owner of its number (created when none exists), when the campaign first reaches them. A longer list continues with `updateCampaign` and `addRecipients`. Nothing happens outside `contactHours` (in `timeZone`, the organization's by default) or outside `scheduledStartAt` and `scheduledEndAt`; each recipient gets up to `maxAttempts` attempts, `retryAfterMinutes` apart, after the results in `retryOn`. Read it with `expand=readiness` to see whether it can start. Starting it (REST `POST /v1/campaigns/{campaignId}/start`) is not an AI tool. Read the `run-a-campaign` skill.
    pub async fn create(&self, input: &types::RaizCampaignsCreateInput) -> Result<types::RaizCampaignsCreateOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_CREATE, input).await
    }

    /// Get a campaign
    ///
    /// Read one campaign: Agent, schedule, contact hours, retries, progress and, in `errorCode`, why it cannot reach anyone now. `expand=recipients` adds its recipients in the order they were added, each with attempts, next attempt, error and latest call or message; continue with `recipientsCursor` (`recipientsDirection` reads the other way). `expand=readiness` says whether it can start and lists what has to be fixed first, without reaching anyone. `expand=results` counts what people answered, by the Agent's data definitions with options; each recipient also says their own. Read it before `update_campaign` and send `expectedVersion`.
    pub async fn get(&self, input: &types::RaizCampaignsGetInput) -> Result<types::RaizCampaignsGetOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_GET, input).await
    }

    /// List campaigns
    ///
    /// List the organization's campaigns, newest first, each with its progress (recipients pending, in progress, completed, failed and succeeded; on WhatsApp also delivered, read and replied). Filter by `channelType`, `status` or `agentId`. A campaign's calls and messages: `list_conversations` with `campaignId`; its numbers by day: `get_report` with `name: campaign_progress`.
    pub async fn list(&self, input: &types::RaizCampaignsListInput) -> Result<types::RaizCampaignsListOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_LIST, input).await
    }

    /// Start a campaign
    ///
    /// Start reaching people: from now on the campaign's due recipients are called, at most `settings.channels.maxConcurrentOutboundCalls` (five by default) at once across the organization, or sent their message, a few a second. A call campaign needs an active, published voice Agent (VOICE_AGENT_NOT_READY). A WhatsApp campaign needs an active text Agent (CAMPAIGN_AGENT_NOT_READY), a line that can send (CHANNEL_NOT_READY), an approved template whose parameters fit (a `TEMPLATE_*` code), a value for every variable (CAMPAIGN_VARIABLE_MISSING) and, when the template's header is a document, image or video, the first pending recipient's own `header` file (CAMPAIGN_HEADER_MISSING). A recipient must be pending (CAMPAIGN_NO_RECIPIENTS). `get_campaign` with `expand=readiness` lists all of this beforehand. A campaign starts once: starting a running one changes nothing, and a paused one resumes with `update_campaign` and `paused: false`. Not an AI tool: it reaches real people.
    pub async fn start(&self, input: &types::RaizCampaignsStartInput) -> Result<types::RaizCampaignsStartOutput, Error> {
        self.0.call(&operations::RAIZ_CAMPAIGNS_START, input).await
    }

    /// Update a campaign
    ///
    /// Change a campaign. Omitted fields stay. `paused: true` pauses a started campaign (no new call or message starts; those in progress finish) and `paused: false` resumes it, with the same checks as starting. Before the start, `recipients` is the complete set, replacing whoever was there; once started it only adds. `addRecipients` adds at any moment, which is how a list longer than one request arrives: a number the campaign already has is skipped, so repeating a part reaches nobody twice, and a recipient's `header` file is stored only when its row has none (CAMPAIGN_HEADER_NOT_STORED says some were not: send those recipients again). The Agent changes only before the start, and a completed campaign accepts only its name and metadata. A change lifts the wait of a campaign held by `errorCode`: fixing a template and sending it here tries again at once.
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
    /// Give a WhatsApp conversation the AI handed to the team back to the AI, which continues the contact's pending request now, though the contact has not written. Review the resolved blocker first; use expectedVersion and expectedHandoffMessageId to refuse a stale decision with VERSION_CONFLICT. `note` preserves the evidence and remaining request for the Agent. Inside WhatsApp's 24-hour window the Agent writes at once; outside it, give `template`: Raíz sends that approved template in the conversation and the Agent answers when the person replies (WHATSAPP_WINDOW_CLOSED without one). Once per handoff, and for an AI handoff. Preserve explicit human requests, unresolved clinical/administrative decisions, identity conflicts and uncertain writes with the team. Not an AI tool: it makes the Agent write to a real person.
    pub async fn r#continue(&self, input: &types::RaizConversationsContinueInput) -> Result<types::RaizConversationsContinueOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_CONTINUE, input).await
    }

    /// Count conversations
    ///
    /// Count the conversations `list_conversations` returns for the same filters (every filter it takes, without `sort`), up to 10,000: `capped` then says at least that many match. `averageMessageCount` and `averageDurationSeconds` are their average length. `groupBy` also divides them, largest group first, each with its outcomes: by `outcome`, `status`, `channel`, `agent`, `sentiment`, `tool` (which Agent tools they used, and in how many a tool failed), `toolError` (why calls of `toolName` failed) or `fact:<definition key>` (such as the reason for contact). Use it to answer how many conversations match, or how they divide, instead of paging through the list; for volumes per day use `get_report`.
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
    /// Find calls, WhatsApp chats and web chats. Filter by `contactId` (as counterpart or subject), `agentId`, `channelId`, `channelType`, `status`, `conversationDirection`, `origin`, `outcome`, `sentiment`, `handling` (`ai` or `team`: whether the AI takes part; `escalated`: handed to the team by the AI), `worker` (`ai` or `team`: who answers it now), the Agent's tools (`toolName`, `toolStatus`: which tool it called and how the call ended), a captured fact (`factDefinition`, `factValue`), `assessment` (`failed` or `passed`), `toReview` (evaluated failures or unresolved handoffs; explicitly resolved successes leave this queue), length (`messagesMin`, `messagesMax`, `durationMin`, `durationMax`), `campaignId`, `profileId`, `requestedBy` and start time (`startedAfter`, `startedBefore`); `q` searches participant identities (names, profile names, usernames, phones, emails and documents), visible messages, titles, summaries and exact conversation references; `count_conversations` counts every match. Each item already carries its title, summary, outcome, sentiment, workers and tool calls (how many, how many failed, the latest failure), often enough to answer. `sort`: `started` (newest first), `last_message` (latest activity first) or `changed`, the feed of status, outcome and analysis changes, oldest first, that systems poll from a saved `pageInfo.nextCursor` (a deleted conversation appears once with `deletedAt`). Next: `get_conversation` for a transcript.
    pub async fn list(&self, input: &types::RaizConversationsListInput) -> Result<types::RaizConversationsListOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_LIST, input).await
    }

    /// Send a message
    ///
    /// Write in an open conversation. In a web conversation you write as the visitor (`role: contact`) and the Agent answers in the background; read the transcript to see the reply. On WhatsApp and in a call you write as a person of the organization (`role: operator`): the message is delivered on the channel and `deliveryStatus` follows it, and on WhatsApp it pauses the Agent for this person until resumed. On WhatsApp a message may carry one image or document (base64, up to 5 MiB) with the text as its caption; elsewhere a file fails with CHANNEL_TYPE_MISMATCH. On WhatsApp `template` sends an approved template of the channel instead, the only message WhatsApp delivers when the person has not written in the last 24 hours; a refused template sends nothing (the TEMPLATE_* codes). Not an AI tool.
    pub async fn send(&self, input: &types::RaizConversationsSendInput) -> Result<types::RaizConversationsSendOutput, Error> {
        self.0.call(&operations::RAIZ_CONVERSATIONS_SEND, input).await
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
    /// Read your organization: name, default language and time zone, deployment capabilities, and the settings in effect (defaults included), such as how many outbound calls may run at once, who is emailed when the AI escalates a conversation or fails in it and how often (each notice at once, or a digest every few hours), and the daily report. capabilities.managedVoiceOutbound reports whether the shared outbound line is configured; settings.channels.managedVoiceOutboundEnabled is the organization's separate permission to use it.
    pub async fn get(&self, input: &types::RaizOrganizationGetInput) -> Result<types::RaizOrganizationGetOutput, Error> {
        self.0.call(&operations::RAIZ_ORGANIZATION_GET, input).await
    }

    /// Send the daily report now
    ///
    /// Email the daily report now to the workspace owners with product access and the additional recipients in `settings.dailyReport`, whether or not it goes out by itself each day: the calls and conversations registered in the last 24 hours, the conversations waiting for the team and those that need a review. It arrives within a minute. Fails with DAILY_REPORT_RECIPIENTS_MISSING when the report has no recipients, and DAILY_REPORT_IN_PROGRESS while a report asked for earlier is still on its way.
    pub async fn send_daily_report(&self, input: &types::RaizOrganizationSendDailyReportInput) -> Result<types::RaizOrganizationSendDailyReportOutput, Error> {
        self.0.call(&operations::RAIZ_ORGANIZATION_SEND_DAILY_REPORT, input).await
    }

    /// Update the organization
    ///
    /// Change the organization's default language, time zone or settings. Edit the organization's name in Arbol Account. Settings merge: an omitted setting stays and `null` returns it to its default; `notifications.recipients` is replaced as a whole. Fails with TIME_ZONE_UNKNOWN when the time zone is not a valid IANA zone (for example `America/Bogota`).
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
    /// Get the numbers behind a question. `name`: `conversations_daily` (conversations the AI took part in, per day), `booking_details` (verified individual creation receipts with patient, specialty, appointments and later recorded cancellations; also requires conversation read/write permission; filter specialty by its recorded code), `booking_outcomes` (verified saved booking receipts by completion day and integration, excluding tests, imports and deleted conversations; createdCount counts new appointments created by the AI, not confirmations of existing appointments; confirmedCount is its compatibility alias and must not be added again; reschedules, cancellations, failures, pending and unverified results separately), `tool_calls_daily` (Agent tool calls per day), `tool_calls` (each call, newest first), `assessment_results_daily` (assessment results per day), `pqrsf_daily` (PQRSF classification, confirmed informational resolution and correct referrals from current pqrsf_* facts; unknown and pending analysis remain visible; excludes tests/imports/deleted records), each over days in the organization's time zone (`from`, `to`; the last 30 days by default), or `campaign_progress`. Narrow with `agentId`, `channelType`, `campaignId` or `definitionId`; unsupported filters fail. At most 1000 rows: `hasMore` means split the range or filter. Booking outcomes return no partial totals when their evidence scan exceeds its bound. Conversation success and completed tool calls never establish a booking; these are historical receipts, not the provider's current appointment status.
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
    /// List stored authorizations
    ///
    /// Page the authorizations already observed for an active IPS connection. Each record retains its source timestamp. This does not refresh the provider. Follow pageInfo.nextCursor for all records.
    pub async fn list(&self, input: &types::SaludTotalAuthorizationsListInput) -> Result<types::SaludTotalAuthorizationsListOutput, Error> {
        self.0.call(&operations::SALUD_TOTAL_AUTHORIZATIONS_LIST, input).await
    }

    /// Consult and store authorizations from the IPS report
    ///
    /// Pass from/to and a permitted site to query the IPS report without a member filter, store each authorization and return one page with its number, provider status, validity and document state. The source permits at most 30 days. Location choices are returned when needed. The legacy member/day input still returns a count. A provider status is preserved verbatim; a filing number is not approval.
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

/// `sanitas.affiliations`
#[derive(Debug, Clone, Copy)]
pub struct SanitasAffiliationsApi<'a>(&'a Client);

impl<'a> SanitasAffiliationsApi<'a> {
    /// Look up affiliation
    ///
    /// Read one matching member contract from an active Account connection. Confirmed affiliation results are reused for seven days. Unknown or unresolved is never inactive. Session cookies are reused; one renewal is admitted only on expiry. This does not authorize care.
    pub async fn lookup(&self, input: &types::SanitasAffiliationsLookupInput) -> Result<types::SanitasAffiliationsLookupOutput, Error> {
        self.0.call(&operations::SANITAS_AFFILIATIONS_LOOKUP, input).await
    }
}

/// `sanitas.connections`
#[derive(Debug, Clone, Copy)]
pub struct SanitasConnectionsApi<'a>(&'a Client);

impl<'a> SanitasConnectionsApi<'a> {
    /// Get connection
    ///
    /// Read safe status. expand providers reads the available business provider contexts with the retained session, without submitting credentials.
    pub async fn get(&self, input: &types::SanitasConnectionsGetInput) -> Result<types::SanitasConnectionsGetOutput, Error> {
        self.0.call(&operations::SANITAS_CONNECTIONS_GET, input).await
    }
}

/// `sanitas`
#[derive(Debug, Clone, Copy)]
pub struct SanitasApi<'a>(&'a Client);

impl<'a> SanitasApi<'a> {
    pub fn affiliations(&self) -> SanitasAffiliationsApi<'a> {
        SanitasAffiliationsApi(self.0)
    }

    pub fn connections(&self) -> SanitasConnectionsApi<'a> {
        SanitasConnectionsApi(self.0)
    }
}

/// `sisben.classifications`
#[derive(Debug, Clone, Copy)]
pub struct SisbenClassificationsApi<'a>(&'a Client);

impl<'a> SisbenClassificationsApi<'a> {
    /// Look up a Sisbén classification
    ///
    /// Read registration and the official group and subgroup. A null registration means the source could not verify the document.
    pub async fn lookup(&self, input: &types::SisbenClassificationsLookupInput) -> Result<types::SisbenClassificationsLookupOutput, Error> {
        self.0.call(&operations::SISBEN_CLASSIFICATIONS_LOOKUP, input).await
    }
}

/// `sisben`
#[derive(Debug, Clone, Copy)]
pub struct SisbenApi<'a>(&'a Client);

impl<'a> SisbenApi<'a> {
    pub fn classifications(&self) -> SisbenClassificationsApi<'a> {
        SisbenClassificationsApi(self.0)
    }
}

/// `sura.affiliations`
#[derive(Debug, Clone, Copy)]
pub struct SuraAffiliationsApi<'a>(&'a Client);

impl<'a> SuraAffiliationsApi<'a> {
    /// Look up eligibility
    ///
    /// Look up affiliate rights by exact document. Verified results for the same document and connection are reused for seven days. Eligible coverage does not authorize a procedure or determine patient charges. A noContract outcome is distinct from inactive or unavailable.
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
    /// Read an existing SaludWeb authorization for the exact patient, issuer and authorization number. Reports its status, dates, the authorized procedures, the provider it is directed to, explicit charge type, fixed fee or copay percentage and provider-reported payments. Procedures that could not be read, unknown balance and copay cap remain null. Does not issue, attend, modify or pay an authorization.
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
    /// Find the free appointment times for one patient and one service. With a date it answers for that day. With no date it answers for the professional's next day with an agenda, which can be full. times[] are the hours you may book, and only free ones are ever listed; otherDates[] are other days that month with room; days[] says how every day of that month stands (available, full or no_agenda); reserved says the day's only free slot is held back; and otherProfessionals[] are the other doctors who see this service. An empty times[] does not mean the whole agenda is full: prepare_appointment searches other months. Offer two or three times, never the whole list.
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

/// `sura.operationTasks`
#[derive(Debug, Clone, Copy)]
pub struct SuraOperationTasksApi<'a>(&'a Client);

impl<'a> SuraOperationTasksApi<'a> {
    /// Cancel a task
    ///
    /// Cancel a task you started that has not started running; one that has runs to its end.
    pub async fn cancel(&self, input: &types::SuraOperationTasksCancelInput) -> Result<types::SuraOperationTasksCancelOutput, Error> {
        self.0.call(&operations::SURA_OPERATION_TASKS_CANCEL, input).await
    }

    /// Read a task
    ///
    /// Read a task you started: whether it still works, then the operation's value or failure.
    pub async fn get(&self, input: &types::SuraOperationTasksGetInput) -> Result<types::SuraOperationTasksGetOutput, Error> {
        self.0.call(&operations::SURA_OPERATION_TASKS_GET, input).await
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

/// `sura.schedules`
#[derive(Debug, Clone, Copy)]
pub struct SuraSchedulesApi<'a>(&'a Client);

impl<'a> SuraSchedulesApi<'a> {
    /// Look up a professional schedule
    ///
    /// Read a professional’s generated schedule and capacity on one date without a patient document. Omit serviceId to inspect all their registered services at the connection’s institution. Distinguishes a full day from no schedule, an ungenerated period, blocked capacity and a generated period not offered. Returns real configured hours and created, offered, assigned, available and blocked counts. Counts are administrative capacity, never patient eligibility or a booking proposal. Does not create periods, change hours, book or cancel appointments. Use bookingOptions.lookup with the patient before offering a bookable slot. A technical failure never means no schedule.
    pub async fn lookup(&self, input: &types::SuraSchedulesLookupInput) -> Result<types::SuraSchedulesLookupOutput, Error> {
        self.0.call(&operations::SURA_SCHEDULES_LOOKUP, input).await
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

    pub fn operation_tasks(&self) -> SuraOperationTasksApi<'a> {
        SuraOperationTasksApi(self.0)
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

    pub fn schedules(&self) -> SuraSchedulesApi<'a> {
        SuraSchedulesApi(self.0)
    }

    pub fn services(&self) -> SuraServicesApi<'a> {
        SuraServicesApi(self.0)
    }
}

/// `visoft.appointmentReceipts`
#[derive(Debug, Clone, Copy)]
pub struct VisoftAppointmentReceiptsApi<'a>(&'a Client);

impl<'a> VisoftAppointmentReceiptsApi<'a> {
    /// Look up an appointment receipt
    ///
    /// SIIPS's own printed receipt (ticket) of one assigned appointment, by the reference visoft.appointments.list answered: the PDF SIIPS's print action serves, unchanged, base64. It is answered only when its text names the appointment's day, hour and patient; otherwise APPOINTMENT_RECEIPT_UNAVAILABLE. Never cached. Printing changes no appointment and sends nothing to the patient.
    pub async fn lookup(&self, input: &types::VisoftAppointmentReceiptsLookupInput) -> Result<types::VisoftAppointmentReceiptsLookupOutput, Error> {
        self.0.call(&operations::VISOFT_APPOINTMENT_RECEIPTS_LOOKUP, input).await
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

    /// List assigned appointments
    ///
    /// Read assigned appointments for one day at a cost centre, ordered by moment and reference, with the patient's document number, the contact phone and the state (`status`) SIIPS records. Never cached. A full upstream page that cannot prove completeness is PROVIDER_UNAVAILABLE; narrow by professionalId rather than treating a partial agenda as complete. This operation changes no appointment and sends no message.
    pub async fn list(&self, input: &types::VisoftAppointmentsListInput) -> Result<types::VisoftAppointmentsListOutput, Error> {
        self.0.call(&operations::VISOFT_APPOINTMENTS_LIST, input).await
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

/// `visoft.patientNameEncodings`
#[derive(Debug, Clone, Copy)]
pub struct VisoftPatientNameEncodingsApi<'a>(&'a Client);

impl<'a> VisoftPatientNameEncodingsApi<'a> {
    /// Audit stored patient name encoding
    ///
    /// Read the current SIIPS records of the supplied patients and identify name fields with recognizable misdecoded Spanish accents. Returns only the affected field names and document type, without the names or document numbers. Requires the permission to correct patient records. This operation only reads records.
    pub async fn lookup(&self, input: &types::VisoftPatientNameEncodingsLookupInput) -> Result<types::VisoftPatientNameEncodingsLookupOutput, Error> {
        self.0.call(&operations::VISOFT_PATIENT_NAME_ENCODINGS_LOOKUP, input).await
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
    pub fn appointment_receipts(&self) -> VisoftAppointmentReceiptsApi<'a> {
        VisoftAppointmentReceiptsApi(self.0)
    }

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

    pub fn patient_name_encodings(&self) -> VisoftPatientNameEncodingsApi<'a> {
        VisoftPatientNameEncodingsApi(self.0)
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
