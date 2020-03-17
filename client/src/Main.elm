module Main exposing (main)

import Browser
import Html exposing (Html, button, div, h3, h4, hr, i, input, li, p, text, ul)
import Html.Attributes exposing (class, disabled, placeholder, value)
import Html.Events exposing (onClick, onInput)
import Http
import Json.Decode exposing (Decoder, field, list, string)


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }


type alias Model =
    { word : String
    , matches : List String
    , errorMessage : Maybe String
    , waitingForResponse : Bool
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { word = ""
      , matches = []
      , errorMessage = Nothing
      , waitingForResponse = False
      }
    , Cmd.none
    )


type Msg
    = SetWord String
    | SearchForMatches
    | DataReceived (Result Http.Error (List String))


url : String
url =
    "http://localhost:3030/word/"


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SetWord value ->
            ( { model | word = value }, Cmd.none )

        SearchForMatches ->
            ( { model | waitingForResponse = True }, Http.get { url = url ++ model.word, expect = Http.expectJson DataReceived matchesDecoder } )

        DataReceived result ->
            case result of
                Ok data ->
                    ( { model | matches = data, errorMessage = Nothing, waitingForResponse = False }, Cmd.none )

                Err _ ->
                    ( { model | errorMessage = Just "Could not get data from server", waitingForResponse = False }, Cmd.none )


view : Model -> Html Msg
view model =
    div [ class "container" ]
        [ h3 [] [ text "Word Unscrambler" ]
        , div [ class "row" ]
            [ div [ class "col s12" ]
                [ div [ class "input-field" ]
                    [ input [ placeholder "Letters to unscramble", value model.word, onInput SetWord ] []
                    ]
                ]
            ]
        , div [ class "row" ]
            [ div [ class "col" ]
                [ button [ class "btn blue darken-1", onClick SearchForMatches, disabled model.waitingForResponse ] [ i [ class "material-icons right" ] [ text "send" ], text "Search" ]
                ]
            , div [ class "col" ]
                [ div
                    [ class
                        ("preloader-wrapper"
                            ++ (if model.waitingForResponse then
                                    " active"

                                else
                                    ""
                               )
                        )
                    ]
                    [ div [ class "spinner-layer spinner-blue-only" ]
                        [ div [ class "circle-clipper left" ] [ div [ class "circle" ] [] ]
                        , div [ class "gap-patch" ] [ div [ class "circle" ] [] ]
                        , div [ class "circle-clipper right" ] []
                        ]
                    ]
                ]
            ]
        , hr [] []
        , h4 [] [ text "Matches" ]
        , p []
            [ ul [ class "collection" ]
                (List.map
                    (\val -> li [ class "collection-item" ] [ text val ])
                    model.matches
                )
            ]
        ]


matchesDecoder : Decoder (List String)
matchesDecoder =
    field "matches" (list string)
