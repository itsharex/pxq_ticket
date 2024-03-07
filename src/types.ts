

export type SendVerificationCodeResult = {
    statusCode: number,
    comments: string,
    data: boolean | null
}

export type GeneratePhotoCodeResult = {
    statusCode: number,
    comments: string,
    data: {
        baseCode: string
    }
}

export type LoginResult = {
    statusCode: number,
    comments: string,
    data: {
        accessToken: string,
        refreshToken: string,
    }
}

export type UserProfileResult = {
    statusCode: number,
    comments: string,
    data: {
        nickname: string,
        avatar: string,
    }
}

export type RefreshTokenResult = {
    statusCode: number,
    comments: string,
    data: {
        accessToken: string,
        refreshToken: string,
    }
}


export type Show = {
    showId: string
    showName: string
    posterUrl: string
    showDate: string
    minOriginalPrice: string
    venueName: string
    showStatus: string
    latestSaleTime: string | null
    cityName: string,
    backendCategory: {
        code: number,
        name: string,
        displayName: string
    }
}

export type QueryShowSessionResult = {
    statusCode: number,
    data: Session[]
}

export type Session = {
    bizShowSessionId: string,
    sessionName: string,
    seatPlans: SeatPlan[],
    sessionSaleTime: number | null,
    sessionStatus: string,
}



export type SeatPlan = {
    seatPlanId: string,
    seatPlanName: string,
    originalPrice: number,
    canBuyCount: number | null

}

export type Audiences = {
    id: string,
    idNo: string,
    name: string,
    description: string
}

export type CurShowData = {
    show: Show,
    session: Session,
    seatPlans: SeatPlan[],
    seatPlanIndex: number,
}

export type GetUserAudiencesResult = {
    statusCode: number,
    comments: string,
    data: Audiences[]
}

export type GetSeatPlansResult = {
    statusCode: number,
    comments: string,
    data: {
        seatPlans: SeatPlan[]
    }
}


export type UserLocation = {
    cityId: string,
    cityName: string,
    provinceId: string,
    provinceName: string,
    siteId: string
}

export type GetUserLocationResult = {
    statusCode: number,
    comments: string,
    data: UserLocation
}

export type BuyTicketParam = {
    show: Show,
    session: Session,
    seatPlan: SeatPlan,
    ticketNum: number,
    audiences: Audiences[],
    isRealname: boolean,
    isExpress: boolean,
    address: Address | null
}

export type NoteInfo = {

    name: string,

    value: string,

    type: boolean,

    code: string
}

export type ShowDetail = {
    rsCode: number
    basicInfo: Show,
    noteInfos: NoteInfo[]
}

export type GetShowDetailResult = {
    statusCode: number,
    comments: string,
    data: ShowDetail
}

export type AddressLocation = {

    location_id: string,

    province: string,

    city: string,

    district: string,
}


export type Address = {
    addressId: string

    username: string,

    cellphone: string,

    detailAddress: string,

    isDefault: boolean,

    location: AddressLocation,
}


export type GetAddressResult = {
    statusCode: number,
    comments: string,
    data: Address[]
}

export type Order = {
    orderId: string,
    orderNumber: string,
    firstShowName: string,
    qty: number,
    displayPosterURL: string,
    payAmount: number,
    orderDetailState: {
        displayName: string
    }
    firstSessionName: string,

    cityName: string,

    showTimeDesc: string,

    firstVenueName: string
}


export type GetOrdersResult = {
    statusCode: number,
    comments: string,
    data: Order[]
}