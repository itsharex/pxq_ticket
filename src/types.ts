

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

export type Order = {
    id: number,
    items: OrderItem[],
}

export type OrderItem = {
    id: number,
    image: string,
    title: string,
    quantity: number,
    price: number,
}


export type Concert = {
    showId: string
    showName: string
    posterUrl: string
    showDate: string
    minOriginalPrice: string
    venueName: string
    showStatus: string
    latestSaleTime: string | null
    cityName: string
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
    show: Concert,
    session: Session,
    seatPlans: SeatPlan[],
    seatPlanIndex: number
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